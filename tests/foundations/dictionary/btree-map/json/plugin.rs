// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-btree-map-json")]
fn roundtrip_btree_map_json(
    value: std::collections::BTreeMap<String, i64>,
) -> Result<std::collections::BTreeMap<String, i64>, String> {
    Ok(value)
}
