// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-linked-list-json")]
fn roundtrip_linked_list_json(
    value: std::collections::LinkedList<i64>,
) -> Result<std::collections::LinkedList<i64>, String> {
    Ok(value)
}
