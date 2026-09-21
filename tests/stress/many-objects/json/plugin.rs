// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Typwire)]
struct StressObjectJson {
    x: i64,
    y: i64,
    label: String,
}

#[typwire::export(format = "json", rename = "stress-many-objects-json")]
fn stress_many_objects_json(
    values: Vec<StressObjectJson>,
) -> Result<Vec<StressObjectJson>, String> {
    Ok(values)
}
