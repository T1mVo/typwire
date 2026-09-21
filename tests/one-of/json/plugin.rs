// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-one-of-json")]
fn roundtrip_one_of_json(
    value: OneOf3<i64, f64, String>,
) -> Result<OneOf3<i64, f64, String>, String> {
    Ok(value)
}
