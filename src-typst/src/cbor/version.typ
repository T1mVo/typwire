#import "utils.typ": is-debug-assert

#let type-name = "version"

/// Encode a version into CBOR-compatible dictionary.
///
/// Only the first five components are considered.
///
/// - version (version): A version.
/// -> dictionary
#let encode(version) = {
  if is-debug-assert {
    assert(
      std.type(version) == std.version,
      message: "version.encode: version must be of type version",
    )
  }

  (
    typwire-type: type-name,
    major: version.at(0),
    minor: version.at(1),
    patch: version.at(2),
    revision: version.at(3),
    build: version.at(4),
  )
}

/// Decode a CBOR-compatible dictionary into a version.
///
/// - value (dictionary): A dictionary representing the `encode`d value.
/// -> version
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "version.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )

    for (index, field) in (
      "major",
      "minor",
      "patch",
      "revision",
      "build",
    ).enumerate() {
      assert(
        type(value.at(field)) == int,
        message: "version.decode: value."
          + field
          + " must be of type int, Found "
          + str(type(value.at(field))),
      )
    }
  }

  version(value.major, value.minor, value.patch, value.revision, value.build)
}
