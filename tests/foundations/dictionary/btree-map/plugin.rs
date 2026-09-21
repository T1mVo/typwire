// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_btree_map(
    value: std::collections::BTreeMap<String, i64>,
) -> Result<std::collections::BTreeMap<String, i64>, String> {
    Ok(value)
}
