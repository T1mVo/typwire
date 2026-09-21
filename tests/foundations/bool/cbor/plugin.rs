// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-bool-cbor")]
fn roundtrip_bool_cbor(value: bool) -> Result<bool, String> {
    Ok(value)
}
