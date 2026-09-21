// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_vec(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}
