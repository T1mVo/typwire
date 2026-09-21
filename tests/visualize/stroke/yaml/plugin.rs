// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-stroke-yaml")]
fn roundtrip_stroke_yaml(value: Stroke) -> Result<Stroke, String> {
    Ok(value)
}
