// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-center-json")]
fn roundtrip_center_json(value: Center) -> Result<Center, String> {
    Ok(value)
}
