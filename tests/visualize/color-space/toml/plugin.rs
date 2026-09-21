// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-color-space-toml")]
fn roundtrip_color_space_toml(value: ColorSpace) -> Result<ColorSpace, String> {
    Ok(value)
}
