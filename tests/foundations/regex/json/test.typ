#import "../../../common.typ": roundtrip-format, plugin, typwire
#let exports = dictionary(plugin)

#roundtrip-format(typwire.json, exports.at("roundtrip-regex-json"), regex("a.*b"))
#roundtrip-format(typwire.json, exports.at("roundtrip-regex-json"), regex(""))

// Regex patterns are full of literal backslashes (unlike a `path`), which
// is exactly the case `unescape-repr-string` needed a real single-pass fix
// for -- see its comment in `utils.typ`.
#roundtrip-format(typwire.json, exports.at("roundtrip-regex-json"), regex("\d+\s*\w"))
#roundtrip-format(typwire.json, exports.at("roundtrip-regex-json"), regex("a\\b"))
#roundtrip-format(typwire.json, exports.at("roundtrip-regex-json"), regex("weird \"quoted\" [a-z]+"))

// The decoded value is a genuinely compiled pattern, not just a string that
// happened to survive the trip.
#assert(typwire.json.call(exports.at("regex-is-match-json"), regex("^\d+$"), "12345"))
#assert(not typwire.json.call(exports.at("regex-is-match-json"), regex("^\d+$"), "12a45"))
