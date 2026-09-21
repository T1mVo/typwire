// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-none-json")]
fn roundtrip_none_json(value: Option<Length>) -> Result<Option<Length>, String> {
    Ok(value)
}
