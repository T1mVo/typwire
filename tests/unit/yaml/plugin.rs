// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-unit-yaml")]
fn roundtrip_unit_yaml(value: ()) -> Result<(), String> {
    Ok(value)
}
