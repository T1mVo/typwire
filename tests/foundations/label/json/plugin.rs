// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-label-json")]
fn roundtrip_label_json(value: Label) -> Result<Label, String> {
    Ok(value)
}
