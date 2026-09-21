// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-hash-set-json")]
fn roundtrip_hash_set_json(
    value: std::collections::HashSet<i64>,
) -> Result<std::collections::HashSet<i64>, String> {
    Ok(value)
}
