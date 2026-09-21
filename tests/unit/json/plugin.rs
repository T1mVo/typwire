// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-unit-json")]
fn roundtrip_unit_json(value: ()) -> Result<(), String> {
    Ok(value)
}
