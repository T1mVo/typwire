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

#[proc_macro_attribute]
pub fn export(arguments: TokenStream, input: TokenStream) -> TokenStream {
    if !arguments.is_empty() {
        return syn::Error::new(
            Span::call_site(),
            "`#[typwire::export]` does not accept arguments",
        )
        .into_compile_error()
        .into();
    }

    let function = parse_macro_input!(input as ItemFn);

    match expand_export(function) {
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

fn expand_export(mut implementation: ItemFn) -> syn::Result<TokenStream2> {
    let typwire = typwire_path()?;
    let arguments = validate_signature(&implementation.sig)?;

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

    let wrapper_arguments = arguments.iter().map(|argument| {
        let attributes = &argument.attributes;
        let name = &argument.name;
        quote! {
            #(#attributes)* #name: &[u8]
        }
    });
    let decoders = arguments.iter().map(|argument| {
        let attributes = &argument.attributes;
        let name = &argument.name;
        let argument_type = &argument.argument_type;
        let argument_name = syn::LitStr::new(&name.to_string(), name.span());
        quote! {
            #(#attributes)*
            let #name: #argument_type = #typwire::__private::decode_argument::<#argument_type>(
                #name,
                #function_name_literal,
                #argument_name,
            )?;
        }
    });
    let call_arguments = arguments.iter().map(|argument| {
        let attributes = &argument.attributes;
        let name = &argument.name;
        quote! {
            #(#attributes)* #name
        }
    });

    Ok(quote! {
        #implementation

        #(#authored_attributes)*
        #[#typwire::__private::wasm_func]
        #visibility fn #function_name(
            #(#wrapper_arguments),*
        ) -> ::core::result::Result<::std::vec::Vec<u8>, ::std::string::String> {
            #(#decoders)*

            let value = #implementation_name(#(#call_arguments),*)
                .map_err(#typwire::__private::display_error)?;

            #typwire::__private::encode_result(&value, #function_name_literal)
        }
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
    if let Some(token) = &signature.unsafety {
        push_error(
            &mut errors,
            syn::Error::new(token.span(), "exported functions cannot be `unsafe`"),
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
