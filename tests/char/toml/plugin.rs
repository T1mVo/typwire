// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-char-toml")]
fn roundtrip_char_toml(value: char) -> Result<char, String> {
    Ok(value)
}
