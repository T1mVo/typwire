// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-regex-toml")]
fn roundtrip_regex_toml(value: Regex) -> Result<Regex, String> {
    Ok(value)
}

// Proves the decoded `Regex` is a genuinely usable, compiled `regex-lite`
// pattern, not just a string carried across the wire.
#[typwire::export(format = "toml", rename = "regex-is-match-toml")]
fn regex_is_match_toml(pattern: Regex, haystack: String) -> Result<bool, String> {
    Ok(pattern.is_match(&haystack))
}
