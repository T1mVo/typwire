// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-regex-json")]
fn roundtrip_regex_json(value: Regex) -> Result<Regex, String> {
    Ok(value)
}

// Proves the decoded `Regex` is a genuinely usable, compiled `regex-lite`
// pattern, not just a string carried across the wire.
#[typwire::export(format = "json", rename = "regex-is-match-json")]
fn regex_is_match_json(pattern: Regex, haystack: String) -> Result<bool, String> {
    Ok(pattern.is_match(&haystack))
}
