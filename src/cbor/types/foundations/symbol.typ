#import "../../utils.typ": is-debug-assert

#let type-name = "symbol"

/// Encode a symbol into a CBOR-compatible dictionary.
///
/// A symbol can carry a whole table of named modifier variants (e.g.
/// `sym.arrow.r` also covers `.double`, `.long`, `.squiggly`, ...), but
/// Typst exposes no way from script to enumerate them -- `str`/`repr` and
/// this crate's own `Serialize` for `Symbol` all only ever expose the
/// *currently selected* glyph (`self.get()`). So only that glyph survives
/// the wire; decoding reconstructs a fresh variant-less symbol from it,
/// which is `==` to the original only when the original had no variants of
/// its own to begin with (most of `sym.*` does).
///
/// - value (symbol): A symbol.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(type(value) == symbol, message: "symbol.encode: value must be of type symbol")
  }

  (
    typwire-type: type-name,
    symbol: str(value),
  )
}

/// Decode a CBOR-compatible dictionary into a symbol.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> symbol
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "symbol.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
    assert(
      type(value.symbol) == str,
      message: "symbol.decode: value.symbol must be of type str, Found " + str(type(value.symbol)),
    )
    assert(
      value.symbol.clusters().len() == 1,
      message: "symbol.decode: value.symbol must be exactly one grapheme cluster",
    )
  }

  symbol(value.symbol)
}
