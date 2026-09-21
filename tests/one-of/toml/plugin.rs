// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-one-of-toml")]
fn roundtrip_one_of_toml(
    value: OneOf3<i64, f64, String>,
) -> Result<OneOf3<i64, f64, String>, String> {
    Ok(value)
}
