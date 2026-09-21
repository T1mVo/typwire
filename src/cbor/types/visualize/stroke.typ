#import "../../utils.typ": is-debug-assert

#let type-name = "stroke"

/// Encode a stroke into a CBOR-compatible dictionary.
///
/// Every field is already one of the wire's other supported shapes (`auto`,
/// a color or gradient, a length, a plain string, `none`, or a plain
/// dictionary of lengths/strings for `dash`) -- so, like `arguments`, this
/// just reads the fields back and leaves recursing into them to the caller.
///
/// - value (stroke): The stroke to encode.
/// -> dictionary
#let encode(value) = {
  if is-debug-assert {
    assert(
      type(value) == stroke,
      message: "stroke.encode: value must be of type stroke",
    )
  }

  (
    typwire-type: type-name,
    paint: value.paint,
    thickness: value.thickness,
    cap: value.cap,
    join: value.join,
    dash: value.dash,
    miter-limit: value.miter-limit,
  )
}

/// Decode a CBOR-compatible dictionary into a stroke.
///
/// - value (dictionary): A dictionary produced by `encode`.
/// -> stroke
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "stroke.decode: value must be a tagged with type " + type-name + ". Found " + value.typwire-type,
    )
  }

  stroke(
    paint: value.paint,
    thickness: value.thickness,
    cap: value.cap,
    join: value.join,
    dash: value.dash,
    miter-limit: value.miter-limit,
  )
}
