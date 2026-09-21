// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-linked-list-cbor")]
fn roundtrip_linked_list_cbor(
    value: std::collections::LinkedList<i64>,
) -> Result<std::collections::LinkedList<i64>, String> {
    Ok(value)
}
