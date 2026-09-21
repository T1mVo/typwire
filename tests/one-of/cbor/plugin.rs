// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-one-of-cbor")]
fn roundtrip_one_of_cbor(
    value: OneOf3<i64, f64, String>,
) -> Result<OneOf3<i64, f64, String>, String> {
    Ok(value)
}
