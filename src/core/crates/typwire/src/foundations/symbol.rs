use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(try_from = "SymbolTypst", into = "SymbolTypst")]
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
struct SymbolTypst {
    typwire_type: String,
    default: String,
    variants: Vec<SymbolVariant>,
}

// Serialize into SymbolTypst
impl From<Symbol> for SymbolTypst {
    fn from(value: Symbol) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            default: value.default,
            variants: value.variants,
        }
    }
}

// Deserialize from SymbolTypst
impl TryFrom<SymbolTypst> for Symbol {
    type Error = String;

    fn try_from(value: SymbolTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Symbol: {}",
                value.typwire_type
            ));
        }

        Ok(Self::with_variants(value.default, value.variants))
    }
}
