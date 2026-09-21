// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_box(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}
