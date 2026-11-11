use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "symbol";

/// A Unicode symbol, optionally with named modifier variants.
///
/// `default` is the selected glyph (what `str(symbol)` gives); `variants` holds
/// the other modifier variants. Glyphs are `String`s because a variant may be
/// more than one scalar value. A rebuilt symbol is never `==` to a built-in
/// `sym.*`/`emoji.*` original in Typst.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/symbol/).
///
/// # Examples
///
/// ```
/// use typwire::{Symbol, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn glyph(symbol: Symbol) -> Result<String, String> {
///     Ok(symbol.default)
/// }
/// ```
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(from = "SymbolTypst<'static>")]
pub struct Symbol {
    pub default: String,
    pub variants: Vec<SymbolVariant>,
}

impl Symbol {
    /// Creates a new `Symbol` instance from the given default glyph, with no
    /// modifier variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Symbol;
    ///
    /// let star = Symbol::new(String::from("★"));
    /// assert_eq!(star.default, "★");
    /// assert!(star.variants.is_empty());
    /// ```
    pub const fn new(default: String) -> Self {
        Self {
            default,
            variants: Vec::new(),
        }
    }

    /// Creates a new `Symbol` instance from the given default glyph and
    /// modifier variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Symbol, SymbolVariant};
    ///
    /// let arrow = Symbol::with_variants(
    ///     String::from("→"),
    ///     vec![SymbolVariant::new(String::from("double"), String::from("⇒"))],
    /// );
    /// assert_eq!(arrow.default, "→");
    /// assert_eq!(arrow.variants.len(), 1);
    /// ```
    pub const fn with_variants(default: String, variants: Vec<SymbolVariant>) -> Self {
        Self { default, variants }
    }
}

/// A single named modifier variant of a [`Symbol`].
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SymbolVariant {
    /// Dot-joined modifier names, e.g. `"double.long"`.
    pub modifiers: String,
    /// The grapheme cluster this variant selects.
    pub glyph: String,
}

impl SymbolVariant {
    /// Creates a new `SymbolVariant` from the given modifiers and glyph.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::SymbolVariant;
    ///
    /// let variant = SymbolVariant::new(String::from("double"), String::from("⇒"));
    /// assert_eq!(variant.modifiers, "double");
    /// assert_eq!(variant.glyph, "⇒");
    /// ```
    pub const fn new(modifiers: String, glyph: String) -> Self {
        Self { modifiers, glyph }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SymbolTypst<'a> {
    typwire_type: Tag<Symbol>,
    default: Cow<'a, str>,
    variants: Cow<'a, [SymbolVariant]>,
}

impl Tagged for Symbol {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing SymbolTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Symbol {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        SymbolTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Symbol> for SymbolTypst<'a> {
    fn from(value: &'a Symbol) -> Self {
        Self {
            typwire_type: Tag::new(),
            default: Cow::Borrowed(&value.default),
            variants: Cow::Borrowed(&value.variants),
        }
    }
}

// Deserialize from SymbolTypst
impl From<SymbolTypst<'_>> for Symbol {
    fn from(value: SymbolTypst<'_>) -> Self {
        Self::with_variants(value.default.into_owned(), value.variants.into_owned())
    }
}
