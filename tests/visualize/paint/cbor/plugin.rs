// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// `ColorGradient` backs Typst's `paint` (fill/stroke) union: a plain color or
// a gradient.
#[typwire::export(rename = "roundtrip-paint-cbor")]
fn roundtrip_paint_cbor(value: ColorGradient) -> Result<ColorGradient, String> {
    Ok(value)
}
