// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-hash-set-toml")]
fn roundtrip_hash_set_toml(
    value: std::collections::HashSet<i64>,
) -> Result<std::collections::HashSet<i64>, String> {
    Ok(value)
}
