// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_color_gradient(value: ColorGradient) -> Result<ColorGradient, String> {
    Ok(value)
}
