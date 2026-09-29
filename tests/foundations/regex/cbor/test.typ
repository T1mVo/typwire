#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-regex-cbor, regex("a.*b"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-regex-cbor, regex(""))

// Regex patterns are full of literal backslashes (unlike a `path`), which
// is exactly the case `unescape-repr-string` needed a real single-pass fix
// for -- see its comment in `utils.typ`.
#roundtrip-format(typwire.cbor, plugin.roundtrip-regex-cbor, regex("\d+\s*\w"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-regex-cbor, regex("a\\b"))
#roundtrip-format(typwire.cbor, plugin.roundtrip-regex-cbor, regex("weird \"quoted\" [a-z]+"))

// The decoded value is a genuinely compiled pattern, not just a string that
// happened to survive the trip.
#assert(typwire.cbor.call(plugin.regex-is-match-cbor, regex("^\d+$"), "12345"))
#assert(not typwire.cbor.call(plugin.regex-is-match-cbor, regex("^\d+$"), "12a45"))
