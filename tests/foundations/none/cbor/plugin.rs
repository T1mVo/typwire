// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-none-cbor")]
fn roundtrip_none_cbor(value: Option<Length>) -> Result<Option<Length>, String> {
    Ok(value)
}
