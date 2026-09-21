/// Replace every `bytes` leaf in a tree produced by `encode.typ::encode`
/// with the tagged array shape `decode.typ::decode-tagged`'s `"bytes"` arm
/// reconstructs: `(typwire-type: "bytes", data: (...))`, `data` a plain
/// array of byte integers -- not base64, since Typst has no built-in base64
/// codec to match it against.
///
/// Only formats with no native byte-string type need this: `cbor.encode`
/// already handles a raw `bytes` value natively, so `format/cbor.typ` skips
/// this step entirely. `format/json.typ`/`yaml.typ`/`toml.typ` all call it
/// on the result of `wire.encode` before handing the tree to their native
/// format encoder.
///
/// - value (any): A tree produced by `wire/encode.typ::encode`.
/// -> any
#let bytes-to-array(value) = {
  if std.type(value) == std.bytes {
    (
      typwire-type: "bytes",
      data: array(value),
    )
  } else if std.type(value) == std.array {
    value.map(bytes-to-array)
  } else if std.type(value) == std.dictionary {
    let result = (:)
    for (key, entry) in value.pairs() {
      result.insert(key, bytes-to-array(entry))
    }
    result
  } else {
    value
  }
}
