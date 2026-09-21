// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-alignment-toml")]
fn roundtrip_alignment_toml(value: Alignment) -> Result<Alignment, String> {
    Ok(value)
}
