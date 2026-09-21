// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-char-json")]
fn roundtrip_char_json(value: char) -> Result<char, String> {
    Ok(value)
}
