// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-btree-set-cbor")]
fn roundtrip_btree_set_cbor(
    value: std::collections::BTreeSet<i64>,
) -> Result<std::collections::BTreeSet<i64>, String> {
    Ok(value)
}
