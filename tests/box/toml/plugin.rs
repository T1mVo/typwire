// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-box-toml")]
fn roundtrip_box_toml(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}
