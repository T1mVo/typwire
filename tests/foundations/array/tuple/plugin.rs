// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_tuple(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
