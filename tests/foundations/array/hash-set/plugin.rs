// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_hash_set(
    value: std::collections::HashSet<i64>,
) -> Result<std::collections::HashSet<i64>, String> {
    Ok(value)
}
