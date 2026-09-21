// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-stop-yaml")]
fn roundtrip_stop_yaml(value: Stop) -> Result<Stop, String> {
    Ok(value)
}
