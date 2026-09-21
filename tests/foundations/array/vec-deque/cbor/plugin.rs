// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-vec-deque-cbor")]
fn roundtrip_vec_deque_cbor(
    value: std::collections::VecDeque<i64>,
) -> Result<std::collections::VecDeque<i64>, String> {
    Ok(value)
}
