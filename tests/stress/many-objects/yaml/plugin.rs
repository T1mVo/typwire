// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Typwire)]
struct StressObjectYaml {
    x: i64,
    y: i64,
    label: String,
}

#[typwire::export(format = "yaml", rename = "stress-many-objects-yaml")]
fn stress_many_objects_yaml(
    values: Vec<StressObjectYaml>,
) -> Result<Vec<StressObjectYaml>, String> {
    Ok(values)
}
