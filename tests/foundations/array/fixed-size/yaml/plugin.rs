// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-fixed-size-array-yaml")]
fn roundtrip_fixed_size_array_yaml(value: [i64; 3]) -> Result<[i64; 3], String> {
    Ok(value)
}
