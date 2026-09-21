// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-char-yaml")]
fn roundtrip_char_yaml(value: char) -> Result<char, String> {
    Ok(value)
}
