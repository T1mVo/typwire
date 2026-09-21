// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-vec-deque-toml")]
fn roundtrip_vec_deque_toml(
    value: std::collections::VecDeque<i64>,
) -> Result<std::collections::VecDeque<i64>, String> {
    Ok(value)
}
