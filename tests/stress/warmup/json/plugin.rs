// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "stress-warmup-json")]
fn stress_warmup_json(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
