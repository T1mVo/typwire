// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// `ColorGradient` backs Typst's `paint` (fill/stroke) union: a plain color or
// a gradient.
#[typwire::export(format = "json", rename = "roundtrip-paint-json")]
fn roundtrip_paint_json(value: ColorGradient) -> Result<ColorGradient, String> {
    Ok(value)
}
