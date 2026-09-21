/// Replace every literal `none` in a tree produced by `encode.typ::encode`
/// with a tagged dictionary `(typwire-type: "none")`. TOML has no way to
/// represent `none`/null at all -- confirmed empirically: Typst's own
/// `toml.encode` silently *drops* a dictionary key whose value is `none`
/// rather than encoding anything, and the `toml` crate's Rust serializer
/// hard-errors on it ("unsupported None value").
///
/// Used only by `format/toml.typ`; JSON/YAML/CBOR represent `none` natively
/// and don't need this.
///
/// - value (any): A tree produced by `wire/encode.typ::encode`.
/// -> any
#let none-to-tagged(value) = {
  if value == none {
    (typwire-type: "none")
  } else if std.type(value) == std.array {
    value.map(none-to-tagged)
  } else if std.type(value) == std.dictionary {
    let result = (:)
    for (key, entry) in value.pairs() {
      result.insert(key, none-to-tagged(entry))
    }
    result
  } else {
    value
  }
}
