// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[derive(Typwire)]` on a user struct.
#[derive(Serialize, Deserialize, Typwire)]
struct PointYaml {
    x: f64,
    y: f64,
    label: String,
}

#[typwire::export(format = "yaml", rename = "roundtrip-struct-yaml")]
fn roundtrip_struct_yaml(value: PointYaml) -> Result<PointYaml, String> {
    Ok(value)
}
