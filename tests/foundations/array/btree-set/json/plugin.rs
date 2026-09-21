// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-btree-set-json")]
fn roundtrip_btree_set_json(
    value: std::collections::BTreeSet<i64>,
) -> Result<std::collections::BTreeSet<i64>, String> {
    Ok(value)
}
