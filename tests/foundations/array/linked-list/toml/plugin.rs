// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-linked-list-toml")]
fn roundtrip_linked_list_toml(
    value: std::collections::LinkedList<i64>,
) -> Result<std::collections::LinkedList<i64>, String> {
    Ok(value)
}
