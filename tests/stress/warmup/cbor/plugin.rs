// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "stress-warmup-cbor")]
fn stress_warmup_cbor(value: Gradient) -> Result<Gradient, String> {
    Ok(value)
}
