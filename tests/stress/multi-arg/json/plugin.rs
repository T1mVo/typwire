// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "stress-multi-arg-json")]
fn stress_multi_arg_json(
    text: String,
    stops: Gradient,
    numbers: Vec<i64>,
) -> Result<(String, Gradient, Vec<i64>), String> {
    Ok((text, stops, numbers))
}
