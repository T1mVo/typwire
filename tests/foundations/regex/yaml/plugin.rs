// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-regex-yaml")]
fn roundtrip_regex_yaml(value: Regex) -> Result<Regex, String> {
    Ok(value)
}

// Proves the decoded `Regex` is a genuinely usable, compiled `regex-lite`
// pattern, not just a string carried across the wire.
#[typwire::export(format = "yaml", rename = "regex-is-match-yaml")]
fn regex_is_match_yaml(pattern: Regex, haystack: String) -> Result<bool, String> {
    Ok(pattern.is_match(&haystack))
}
