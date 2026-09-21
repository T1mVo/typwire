// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-any-json")]
fn roundtrip_any_json(value: Any) -> Result<Any, String> {
    Ok(value)
}
