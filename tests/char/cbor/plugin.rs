// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-char-cbor")]
fn roundtrip_char_cbor(value: char) -> Result<char, String> {
    Ok(value)
}
