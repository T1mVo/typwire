// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-auto-json")]
fn roundtrip_auto_json(value: Auto) -> Result<Auto, String> {
    Ok(value)
}
