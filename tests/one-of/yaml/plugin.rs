// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-one-of-yaml")]
fn roundtrip_one_of_yaml(
    value: OneOf3<i64, f64, String>,
) -> Result<OneOf3<i64, f64, String>, String> {
    Ok(value)
}
