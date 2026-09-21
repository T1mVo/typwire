// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-alignment-yaml")]
fn roundtrip_alignment_yaml(value: Alignment) -> Result<Alignment, String> {
    Ok(value)
}
