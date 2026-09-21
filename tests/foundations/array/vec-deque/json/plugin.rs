// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-vec-deque-json")]
fn roundtrip_vec_deque_json(
    value: std::collections::VecDeque<i64>,
) -> Result<std::collections::VecDeque<i64>, String> {
    Ok(value)
}
