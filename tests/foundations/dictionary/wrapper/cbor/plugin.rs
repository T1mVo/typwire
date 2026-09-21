// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-dictionary-wrapper-cbor")]
fn roundtrip_dictionary_wrapper_cbor(
    value: typwire::Dictionary<i64>,
) -> Result<typwire::Dictionary<i64>, String> {
    Ok(value)
}
