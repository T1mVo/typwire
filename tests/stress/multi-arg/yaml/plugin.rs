// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "stress-multi-arg-yaml")]
fn stress_multi_arg_yaml(
    text: String,
    stops: Gradient,
    numbers: Vec<i64>,
) -> Result<(String, Gradient, Vec<i64>), String> {
    Ok((text, stops, numbers))
}
