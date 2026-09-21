// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_linked_list(
    value: std::collections::LinkedList<i64>,
) -> Result<std::collections::LinkedList<i64>, String> {
    Ok(value)
}
