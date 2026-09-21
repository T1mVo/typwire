// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[derive(Typwire)]` on a user struct.
#[derive(Serialize, Deserialize, Typwire)]
struct PointToml {
    x: f64,
    y: f64,
    label: String,
}

#[typwire::export(format = "toml", rename = "roundtrip-struct-toml")]
fn roundtrip_struct_toml(value: PointToml) -> Result<PointToml, String> {
    Ok(value)
}
