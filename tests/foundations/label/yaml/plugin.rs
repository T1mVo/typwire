// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-label-yaml")]
fn roundtrip_label_yaml(value: Label) -> Result<Label, String> {
    Ok(value)
}
