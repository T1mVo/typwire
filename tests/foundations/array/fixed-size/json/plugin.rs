// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-fixed-size-array-json")]
fn roundtrip_fixed_size_array_json(value: [i64; 3]) -> Result<[i64; 3], String> {
    Ok(value)
}
