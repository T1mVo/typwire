// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-length-json")]
fn roundtrip_length_json(value: Length) -> Result<Length, String> {
    Ok(value)
}
