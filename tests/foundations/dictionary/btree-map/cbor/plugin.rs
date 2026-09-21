// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-btree-map-cbor")]
fn roundtrip_btree_map_cbor(
    value: std::collections::BTreeMap<String, i64>,
) -> Result<std::collections::BTreeMap<String, i64>, String> {
    Ok(value)
}
