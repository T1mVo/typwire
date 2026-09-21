// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-stroke-json")]
fn roundtrip_stroke_json(value: Stroke) -> Result<Stroke, String> {
    Ok(value)
}
