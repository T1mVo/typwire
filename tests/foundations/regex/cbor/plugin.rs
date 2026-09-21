// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-regex-cbor")]
fn roundtrip_regex_cbor(value: Regex) -> Result<Regex, String> {
    Ok(value)
}

// Proves the decoded `Regex` is a genuinely usable, compiled `regex-lite`
// pattern, not just a string carried across the wire.
#[typwire::export(rename = "regex-is-match-cbor")]
fn regex_is_match_cbor(pattern: Regex, haystack: String) -> Result<bool, String> {
    Ok(pattern.is_match(&haystack))
}
