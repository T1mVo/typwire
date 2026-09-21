// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-type-cbor")]
fn roundtrip_type_cbor(value: Type) -> Result<Type, String> {
    Ok(value)
}
