// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-fixed-size-array-toml")]
fn roundtrip_fixed_size_array_toml(value: [i64; 3]) -> Result<[i64; 3], String> {
    Ok(value)
}
