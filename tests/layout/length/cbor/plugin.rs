// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-length-cbor")]
fn roundtrip_length_cbor(value: Length) -> Result<Length, String> {
    Ok(value)
}
