// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "stress-multi-arg-toml")]
fn stress_multi_arg_toml(
    text: String,
    stops: Gradient,
    numbers: Vec<i64>,
) -> Result<(String, Gradient, Vec<i64>), String> {
    Ok((text, stops, numbers))
}
