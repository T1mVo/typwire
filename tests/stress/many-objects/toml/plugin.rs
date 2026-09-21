// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Typwire)]
struct StressObjectToml {
    x: i64,
    y: i64,
    label: String,
}

#[typwire::export(format = "toml", rename = "stress-many-objects-toml")]
fn stress_many_objects_toml(
    values: Vec<StressObjectToml>,
) -> Result<Vec<StressObjectToml>, String> {
    Ok(values)
}
