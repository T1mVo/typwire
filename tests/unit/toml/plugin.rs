// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-unit-toml")]
fn roundtrip_unit_toml(value: ()) -> Result<(), String> {
    Ok(value)
}
