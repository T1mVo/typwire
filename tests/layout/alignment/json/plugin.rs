// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-alignment-json")]
fn roundtrip_alignment_json(value: Alignment) -> Result<Alignment, String> {
    Ok(value)
}
