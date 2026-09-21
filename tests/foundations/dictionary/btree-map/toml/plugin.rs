// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-btree-map-toml")]
fn roundtrip_btree_map_toml(
    value: std::collections::BTreeMap<String, i64>,
) -> Result<std::collections::BTreeMap<String, i64>, String> {
    Ok(value)
}
