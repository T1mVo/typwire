// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-center-toml")]
fn roundtrip_center_toml(value: Center) -> Result<Center, String> {
    Ok(value)
}
