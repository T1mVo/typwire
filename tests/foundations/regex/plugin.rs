// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_regex(value: Regex) -> Result<Regex, String> {
    Ok(value)
}

// Proves the decoded `Regex` is a genuinely usable, compiled `regex-lite`
// pattern, not just a string carried across the wire.
#[typwire::export]
fn regex_is_match(pattern: Regex, haystack: String) -> Result<bool, String> {
    Ok(pattern.is_match(&haystack))
}
