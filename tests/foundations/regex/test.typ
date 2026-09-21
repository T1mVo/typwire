#import "../../common.typ": roundtrip, plugin, typwire

#roundtrip(plugin.roundtrip_regex, regex("a.*b"))
#roundtrip(plugin.roundtrip_regex, regex(""))

// Regex patterns are full of literal backslashes (unlike a `path`), which
// is exactly the case `unescape-repr-string` needed a real single-pass fix
// for -- see its comment in `utils.typ`.
#roundtrip(plugin.roundtrip_regex, regex("\d+\s*\w"))
#roundtrip(plugin.roundtrip_regex, regex("a\\b"))
#roundtrip(plugin.roundtrip_regex, regex("weird \"quoted\" [a-z]+"))

// The decoded value is a genuinely compiled pattern, not just a string that
// happened to survive the trip.
#assert(typwire.call(plugin.regex_is_match, regex("^\d+$"), "12345"))
#assert(not typwire.call(plugin.regex_is_match, regex("^\d+$"), "12a45"))
