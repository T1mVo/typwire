// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-stroke-toml")]
fn roundtrip_stroke_toml(value: Stroke) -> Result<Stroke, String> {
    Ok(value)
}
