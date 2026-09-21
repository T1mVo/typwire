// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-any-cbor")]
fn roundtrip_any_cbor(value: Any) -> Result<Any, String> {
    Ok(value)
}
