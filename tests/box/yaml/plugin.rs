// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-box-yaml")]
fn roundtrip_box_yaml(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}
