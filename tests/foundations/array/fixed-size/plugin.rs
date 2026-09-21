// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_fixed_size_array(value: [i64; 3]) -> Result<[i64; 3], String> {
    Ok(value)
}
