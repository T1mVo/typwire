use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "symbol";

/// A structure representing a Typst Unicode symbol.
///
/// A Typst `symbol` can carry a whole table of named modifier variants
/// (e.g. `sym.arrow.r` also covers `.double`, `.long`, `.squiggly`, ...),
/// but Typst exposes no way to enumerate them from script -- only the
/// *currently selected* glyph is ever observable (via `str`, `repr`, or
/// Typst's own `Serialize` impl for `Symbol`, which all only expose that
/// one glyph). So only that glyph crosses the wire; reconstructing a
/// `Symbol` value from this loses any other variants the original had.
///
/// Holds a `String`, not a `char`: Typst guarantees a symbol's text is
/// exactly one grapheme cluster, not necessarily one Unicode scalar value
/// (e.g. some arrow variants are a base character plus a variation
/// selector).
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(try_from = "SymbolCbor", into = "SymbolCbor")]
pub struct Symbol {
    pub symbol: String,
}

impl Symbol {
    /// Creates a new `Symbol` instance from the given grapheme cluster.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Symbol;
    ///
    /// let star = Symbol::new(String::from("★"));
    /// assert_eq!(star.symbol, "★");
    /// ```
    pub const fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SymbolCbor {
    typwire_type: String,
    symbol: String,
}

// Serialize into SymbolCbor
impl From<Symbol> for SymbolCbor {
    fn from(value: Symbol) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            symbol: value.symbol,
        }
    }
}

// Deserialize from SymbolCbor
impl TryFrom<SymbolCbor> for Symbol {
    type Error = String;

    fn try_from(value: SymbolCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Symbol: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.symbol))
    }
}
