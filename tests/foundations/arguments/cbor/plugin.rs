// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-arguments-cbor")]
fn roundtrip_arguments_cbor(value: Arguments) -> Result<Arguments, String> {
    Ok(value)
}
