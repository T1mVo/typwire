// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-hash-map-toml")]
fn roundtrip_hash_map_toml(
    value: std::collections::HashMap<String, i64>,
) -> Result<std::collections::HashMap<String, i64>, String> {
    Ok(value)
}
