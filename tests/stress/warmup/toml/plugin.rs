// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "stress-warmup-toml")]
fn stress_warmup_toml(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
