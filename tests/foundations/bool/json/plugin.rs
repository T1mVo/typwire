// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-bool-json")]
fn roundtrip_bool_json(value: bool) -> Result<bool, String> {
    Ok(value)
}
