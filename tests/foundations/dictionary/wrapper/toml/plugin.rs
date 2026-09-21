// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-dictionary-wrapper-toml")]
fn roundtrip_dictionary_wrapper_toml(
    value: typwire::Dictionary<i64>,
) -> Result<typwire::Dictionary<i64>, String> {
    Ok(value)
}
