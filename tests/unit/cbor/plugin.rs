// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-unit-cbor")]
fn roundtrip_unit_cbor(value: ()) -> Result<(), String> {
    Ok(value)
}
