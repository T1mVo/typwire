// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-box-json")]
fn roundtrip_box_json(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}
