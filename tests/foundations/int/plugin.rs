// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_int(value: i64) -> Result<i64, String> {
    Ok(value)
}
