use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "symbol";

/// A structure representing a Typst Unicode symbol.
///
/// A Typst `symbol` can carry a whole table of named modifier variants
/// (e.g. `sym.arrow.r` also covers `.double`, `.long`, `.squiggly`, ...).
/// `default` is the currently selected glyph (what `str`/`Display` on the
/// Typst value gives); `variants` is every other named modifier variant,
/// extracted from Typst's `repr` of the symbol (the only place that table
/// is observable from script at all). Reconstructing a Typst `symbol` value
/// from this is functionally equivalent to the original -- `str()` and
/// modifier field access both agree with it -- but is never `==` to a
/// built-in (`sym.*`/`emoji.*`) original, since those use a different
/// internal representation in Typst than anything a plugin can construct;
/// see `src/cbor/types/foundations/symbol.typ`.
///
/// Holds `String`s, not `char`s: Typst guarantees a variant's text is
/// exactly one grapheme cluster, not necessarily one Unicode scalar value
/// (e.g. some arrow variants are a base character plus a variation
/// selector).
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
