// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[derive(Typwire)]` on a user struct.
#[derive(Serialize, Deserialize, Typwire)]
struct PointJson {
    x: f64,
    y: f64,
    label: String,
}

#[typwire::export(format = "json", rename = "roundtrip-struct-json")]
fn roundtrip_struct_json(value: PointJson) -> Result<PointJson, String> {
    Ok(value)
}
