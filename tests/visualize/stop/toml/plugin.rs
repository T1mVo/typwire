// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-stop-toml")]
fn roundtrip_stop_toml(value: Stop) -> Result<Stop, String> {
    Ok(value)
}
