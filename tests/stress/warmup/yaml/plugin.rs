// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "stress-warmup-yaml")]
fn stress_warmup_yaml(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
