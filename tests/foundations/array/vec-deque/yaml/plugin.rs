// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-vec-deque-yaml")]
fn roundtrip_vec_deque_yaml(
    value: std::collections::VecDeque<i64>,
) -> Result<std::collections::VecDeque<i64>, String> {
    Ok(value)
}
