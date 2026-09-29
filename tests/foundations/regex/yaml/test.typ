#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-regex-yaml, regex("a.*b"))
#roundtrip-format(typwire.yaml, plugin.roundtrip-regex-yaml, regex(""))

// Regex patterns are full of literal backslashes (unlike a `path`), which
// is exactly the case `unescape-repr-string` needed a real single-pass fix
// for -- see its comment in `utils.typ`.
#roundtrip-format(typwire.yaml, plugin.roundtrip-regex-yaml, regex("\d+\s*\w"))
#roundtrip-format(typwire.yaml, plugin.roundtrip-regex-yaml, regex("a\\b"))
#roundtrip-format(typwire.yaml, plugin.roundtrip-regex-yaml, regex("weird \"quoted\" [a-z]+"))

// The decoded value is a genuinely compiled pattern, not just a string that
// happened to survive the trip.
#assert(typwire.yaml.call(plugin.regex-is-match-yaml, regex("^\d+$"), "12345"))
#assert(not typwire.yaml.call(plugin.regex-is-match-yaml, regex("^\d+$"), "12a45"))
