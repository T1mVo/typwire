// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-dictionary-wrapper-json")]
fn roundtrip_dictionary_wrapper_json(
    value: typwire::Dictionary<i64>,
) -> Result<typwire::Dictionary<i64>, String> {
    Ok(value)
}
