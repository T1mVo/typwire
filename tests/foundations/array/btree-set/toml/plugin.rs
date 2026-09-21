// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-btree-set-toml")]
fn roundtrip_btree_set_toml(
    value: std::collections::BTreeSet<i64>,
) -> Result<std::collections::BTreeSet<i64>, String> {
    Ok(value)
}
