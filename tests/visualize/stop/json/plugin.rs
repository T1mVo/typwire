// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-stop-json")]
fn roundtrip_stop_json(value: Stop) -> Result<Stop, String> {
    Ok(value)
}
