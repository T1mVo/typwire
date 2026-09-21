// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_hash_map(
    value: std::collections::HashMap<String, i64>,
) -> Result<std::collections::HashMap<String, i64>, String> {
    Ok(value)
}
