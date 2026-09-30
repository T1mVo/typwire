use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    Attribute, FnArg, GenericArgument, ItemFn, Pat, PathArguments, ReturnType, Signature, Type,
    TypeReference, parse_macro_input,
};

#[proc_macro_derive(Typwire)]
pub fn derive_typwire(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    match expand_derive(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

/// Turn a `fn(A, B, ..) -> Result<T, E>` into a Typst plugin function.
///
/// Options:
/// - `rename = "wire-name"` — export under `wire-name` instead of the Rust
///   function name. The name need not be a valid identifier; reach a
///   non-identifier export from Typst via `dictionary(plugin).at("wire-name")`.
/// - `format = "json"` — decode/encode arguments and the result through this
///   wire format instead of the default `"cbor"`. One of `"cbor"`, `"json"`.
///   Picking a format whose crate feature isn't enabled on the `typwire`
///   dependency is a compile error (the marker type doesn't exist), not a
///   runtime one — the same property `Regex`/`Decimal` already have when
///   their feature is off.
#[proc_macro_attribute]
pub fn export(arguments: TokenStream, input: TokenStream) -> TokenStream {
    let mut rename = None;
    let mut format = None;
    if !arguments.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("rename") {
                let value: syn::LitStr = meta.value()?.parse()?;
                if value.value().is_empty() {
                    return Err(meta.error("`rename` must not be empty"));
                }
                rename = Some(value);
                Ok(())
            } else if meta.path.is_ident("format") {
                let value: syn::LitStr = meta.value()?.parse()?;
                if !matches!(value.value().as_str(), "cbor" | "json") {
                    return Err(meta.error("`format` must be one of \"cbor\", \"json\""));
                }
                format = Some(value);
                Ok(())
            } else {
                Err(meta.error(
                    "unknown option; `#[typwire::export]` only accepts `rename = \"...\"` and `format = \"...\"`",
                ))
            }
        });

        if let Err(error) = syn::parse::Parser::parse(parser, arguments) {
            return error.into_compile_error().into();
        }
    }

    let function = parse_macro_input!(input as ItemFn);

    match expand_export(function, rename, format) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_derive(input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    let typwire = typwire_path()?;
    let name = input.ident;
    let type_generics = input.generics.split_for_impl().1.to_token_stream();
    let mut generics = input.generics;
    generics
        .make_where_clause()
        .predicates
        .push(syn::parse_quote! {
            #name #type_generics:
                #typwire::__private::Serialize + #typwire::__private::DeserializeOwned
        });
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #typwire::Typwire for #name #type_generics #where_clause {}
    })
}

fn expand_export(
    mut implementation: ItemFn,
    rename: Option<syn::LitStr>,
    format: Option<syn::LitStr>,
) -> syn::Result<TokenStream2> {
    let typwire = typwire_path()?;
    let arguments = validate_signature(&implementation.sig)?;
    let format_marker = format_marker(&typwire, format.as_ref());

    let function_name = implementation.sig.ident.clone();
    let function_name_literal = syn::LitStr::new(&function_name.to_string(), function_name.span());
    let implementation_name = implementation_name(&implementation);
    let visibility = implementation.vis.clone();

    let authored_attributes: Vec<_> = implementation
        .attrs
        .iter()
        .filter(|attribute| !is_wasm_func(attribute))
        .cloned()
        .collect();
    let implementation_attributes: Vec<_> = authored_attributes
        .iter()
        .filter(|attribute| is_conditional(attribute))
        .cloned()
        .collect();

    implementation.attrs = implementation_attributes;
    implementation.vis = syn::Visibility::Inherited;
    implementation.sig.ident = implementation_name.clone();

    let wrapper_arguments: Vec<_> = arguments
        .iter()
        .map(|argument| {
            let attributes = &argument.attributes;
            let name = &argument.name;
            quote! {
                #(#attributes)* #name: &[u8]
            }
        })
        .collect();
    let decoders: Vec<_> = arguments
        .iter()
        .map(|argument| {
            let attributes = &argument.attributes;
            let name = &argument.name;
            let argument_type = &argument.argument_type;
            let argument_name = syn::LitStr::new(&name.to_string(), name.span());
            quote! {
                #(#attributes)*
                let #name: #argument_type = #typwire::__private::decode_argument::<#format_marker, #argument_type>(
                    #name,
                    #function_name_literal,
                    #argument_name,
                )?;
            }
        })
        .collect();
    let call_arguments: Vec<_> = arguments
        .iter()
        .map(|argument| {
            let attributes = &argument.attributes;
            let name = &argument.name;
            quote! {
                #(#attributes)* #name
            }
        })
        .collect();

    // Shared body: decode every argument, call the hidden implementation, encode
    // the result. Returns `Result<Vec<u8>, String>` either way.
    let body = quote! {
        #(#decoders)*

        let value = #implementation_name(#(#call_arguments),*)
            .map_err(#typwire::__private::display_error)?;

        #typwire::__private::encode_result::<#format_marker, _>(&value, #function_name_literal)
    };

    let wire = match &rename {
        // Default: let `wasm-minimal-protocol` name and export the wrapper after
        // the Rust function.
        None => quote! {
            #(#authored_attributes)*
            #[#typwire::__private::wasm_func]
            #visibility fn #function_name(
                #(#wrapper_arguments),*
            ) -> ::core::result::Result<::std::vec::Vec<u8>, ::std::string::String> {
                #body
            }
        },
        // `rename = "..."`: `#[wasm_func]` always exports under the wrapper's
        // identifier, and the requested name need not be a valid identifier
        // (kebab-case is the Typst norm), so emit the plugin-protocol glue
        // directly with an explicit `export_name`. The only external contract
        // used here is the Typst-defined `typst_env` import module, which
        // `initiate_protocol!()` also relies on.
        Some(export_name) => {
            let wire_name = format_ident!("{implementation_name}_wire");
            let length_parameters = arguments.iter().map(|argument| {
                let attributes = &argument.attributes;
                let length = format_ident!("__typwire_len_{}", argument.name);
                quote! { #(#attributes)* #length: usize }
            });
            let length_terms = arguments.iter().map(|argument| {
                let attributes = &argument.attributes;
                let length = format_ident!("__typwire_len_{}", argument.name);
                quote! { #(#attributes)* { __typwire_total += #length; } }
            });
            let split_bindings = arguments.iter().map(|argument| {
                let attributes = &argument.attributes;
                let name = &argument.name;
                let length = format_ident!("__typwire_len_{}", argument.name);
                quote! {
                    #(#attributes)*
                    let #name: &[u8] = {
                        let __typwire_start = __typwire_offset;
                        __typwire_offset += #length;
                        &__typwire_buffer[__typwire_start..__typwire_offset]
                    };
                }
            });

            quote! {
                #(#authored_attributes)*
                #[unsafe(export_name = #export_name)]
                #[allow(unused_mut, unused_variables, unused_assignments)]
                extern "C" fn #wire_name(#(#length_parameters),*) -> i32 {
                    #[link(wasm_import_module = "typst_env")]
                    unsafe extern "C" {
                        #[link_name = "wasm_minimal_protocol_write_args_to_buffer"]
                        fn __typwire_write_args(pointer: *mut u8);
                        #[link_name = "wasm_minimal_protocol_send_result_to_host"]
                        fn __typwire_send_result(pointer: *const u8, length: usize);
                    }

                    fn __typwire_dispatch(
                        #(#wrapper_arguments),*
                    ) -> ::core::result::Result<::std::vec::Vec<u8>, ::std::string::String> {
                        #body
                    }

                    let mut __typwire_total: usize = 0;
                    #(#length_terms)*
                    let mut __typwire_buffer = ::std::vec![0u8; __typwire_total];
                    if __typwire_total != 0 {
                        unsafe { __typwire_write_args(__typwire_buffer.as_mut_ptr()); }
                    }

                    let mut __typwire_offset: usize = 0;
                    #(#split_bindings)*

                    let (__typwire_message, __typwire_code): (::std::vec::Vec<u8>, i32) =
                        match __typwire_dispatch(#(#call_arguments),*) {
                            ::core::result::Result::Ok(bytes) => (bytes, 0),
                            ::core::result::Result::Err(error) => (error.into_bytes(), 1),
                        };
                    unsafe {
                        __typwire_send_result(
                            __typwire_message.as_ptr(),
                            __typwire_message.len(),
                        );
                    }
                    __typwire_code
                }
            }
        }
    };

    Ok(quote! {
        #implementation

        #wire
    })
}

struct ExportArgument {
    attributes: Vec<Attribute>,
    name: syn::Ident,
    argument_type: Box<Type>,
}

fn validate_signature(signature: &Signature) -> syn::Result<Vec<ExportArgument>> {
    let mut errors = None;

    if let Some(token) = &signature.constness {
        push_error(
            &mut errors,
            syn::Error::new(token.span(), "exported functions cannot be `const`"),
        );
    }
    if let Some(token) = &signature.asyncness {
        push_error(
            &mut errors,
            syn::Error::new(token.span(), "exported functions cannot be `async`"),
        );
    }
    if let syn::Safety::Unsafe(token) = &signature.safety {
        push_error(
            &mut errors,
            syn::Error::new(token.span, "exported functions cannot be `unsafe`"),
        );
    }
    if let Some(abi) = &signature.abi {
        push_error(
            &mut errors,
            syn::Error::new(
                abi.extern_token.span(),
                "exported functions cannot have an `extern` ABI",
            ),
        );
    }
    if let Some(variadic) = &signature.variadic {
        push_error(
            &mut errors,
            syn::Error::new(variadic.span(), "exported functions cannot be variadic"),
        );
    }
    if !signature.generics.params.is_empty() {
        push_error(
            &mut errors,
            syn::Error::new(
                signature.generics.span(),
                "exported functions cannot have generic parameters",
            ),
        );
    }

    validate_result_return(&signature.output, &mut errors);

    let mut arguments = Vec::with_capacity(signature.inputs.len());
    for input in &signature.inputs {
        match input {
            FnArg::Receiver(receiver) => push_error(
                &mut errors,
                syn::Error::new(
                    receiver.span(),
                    "`#[typwire::export]` can only be used on free functions; methods and receivers are not supported",
                ),
            ),
            FnArg::Typed(argument) => {
                let Pat::Ident(pattern) = argument.pat.as_ref() else {
                    push_error(
                        &mut errors,
                        syn::Error::new(
                            argument.pat.span(),
                            "exported function arguments must use plain identifier patterns",
                        ),
                    );
                    continue;
                };

                if pattern.by_ref.is_some() || pattern.subpat.is_some() {
                    push_error(
                        &mut errors,
                        syn::Error::new(
                            pattern.span(),
                            "exported function arguments must use plain identifier patterns by value",
                        ),
                    );
                    continue;
                }

                if let Some(reference_span) = first_reference_span(&argument.ty) {
                    push_error(
                        &mut errors,
                        syn::Error::new(
                            reference_span,
                            "exported function argument types must be owned; references are not supported",
                        ),
                    );
                    continue;
                }

                arguments.push(ExportArgument {
                    attributes: argument.attrs.clone(),
                    name: pattern.ident.clone(),
                    argument_type: argument.ty.clone(),
                });
            }
        }
    }

    match errors {
        Some(errors) => Err(errors),
        None => Ok(arguments),
    }
}

fn validate_result_return(output: &ReturnType, errors: &mut Option<syn::Error>) {
    let ReturnType::Type(_, return_type) = output else {
        push_error(
            errors,
            syn::Error::new(
                output.span(),
                "exported functions must have an explicit `Result<T, E>` return type",
            ),
        );
        return;
    };

    let Type::Path(result_path) = return_type.as_ref() else {
        push_error(
            errors,
            syn::Error::new(
                return_type.span(),
                "the return type of an exported function must be `Result<T, E>`",
            ),
        );
        return;
    };

    let Some(result_segment) = result_path.path.segments.last() else {
        push_error(
            errors,
            syn::Error::new(
                return_type.span(),
                "the return type of an exported function must be `Result<T, E>`",
            ),
        );
        return;
    };

    let valid_result = result_path.qself.is_none()
        && result_segment.ident == "Result"
        && matches!(
            &result_segment.arguments,
            PathArguments::AngleBracketed(arguments)
                if arguments.args.len() == 2
                    && arguments.args.iter().all(|argument| matches!(argument, GenericArgument::Type(_)))
        );

    if !valid_result {
        push_error(
            errors,
            syn::Error::new(
                return_type.span(),
                "the return type of an exported function must be `Result<T, E>`",
            ),
        );
    }
}

fn first_reference_span(argument_type: &Type) -> Option<Span> {
    struct ReferenceVisitor {
        span: Option<Span>,
    }

    impl<'ast> Visit<'ast> for ReferenceVisitor {
        fn visit_type_reference(&mut self, reference: &'ast TypeReference) {
            if self.span.is_none() {
                self.span = Some(reference.and_token.span());
            }
        }
    }

    let mut visitor = ReferenceVisitor { span: None };
    visitor.visit_type(argument_type);
    visitor.span
}

fn push_error(errors: &mut Option<syn::Error>, error: syn::Error) {
    if let Some(errors) = errors {
        errors.combine(error);
    } else {
        *errors = Some(error);
    }
}

fn is_wasm_func(attribute: &Attribute) -> bool {
    attribute
        .path()
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "wasm_func")
}

fn is_conditional(attribute: &Attribute) -> bool {
    attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr")
}

fn implementation_name(function: &ItemFn) -> syn::Ident {
    let mut hasher = DefaultHasher::new();
    function.sig.to_token_stream().to_string().hash(&mut hasher);
    function
        .block
        .to_token_stream()
        .to_string()
        .hash(&mut hasher);
    let hash = hasher.finish();

    format_ident!(
        "__typwire_export_impl_{}_{hash:016x}",
        function.sig.ident,
        span = function.sig.ident.span()
    )
}

/// Resolve the `format = "..."` option to the marker type it names
/// (`typwire::Cbor` by default), each an ordinary path under the `typwire`
/// crate root -- `syn::meta`'s own validation already rejected anything
/// besides the two recognized names before this runs.
fn format_marker(typwire: &TokenStream2, format: Option<&syn::LitStr>) -> TokenStream2 {
    let marker = match format.map(syn::LitStr::value).as_deref() {
        None | Some("cbor") => format_ident!("Cbor"),
        Some("json") => format_ident!("Json"),
        Some(_) => unreachable!("validated in `export`'s option parser"),
    };
    quote! { #typwire::#marker }
}

fn typwire_path() -> syn::Result<TokenStream2> {
    match crate_name("typwire") {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let normalized_name = name.replace('-', "_");
            let identifier = syn::Ident::new(&normalized_name, Span::call_site());
            Ok(quote!(::#identifier))
        }
        Err(error) => Err(syn::Error::new(
            Span::call_site(),
            format!("could not resolve the `typwire` crate: {error}"),
        )),
    }
}
