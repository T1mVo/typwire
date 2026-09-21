// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-color-toml")]
fn roundtrip_color_toml(value: Color) -> Result<Color, String> {
    Ok(value)
}
