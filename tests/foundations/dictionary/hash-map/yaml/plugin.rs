// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-hash-map-yaml")]
fn roundtrip_hash_map_yaml(
    value: std::collections::HashMap<String, i64>,
) -> Result<std::collections::HashMap<String, i64>, String> {
    Ok(value)
}
