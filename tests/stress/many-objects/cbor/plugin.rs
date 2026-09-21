// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Typwire)]
struct StressObjectCbor {
    x: i64,
    y: i64,
    label: String,
}

#[typwire::export(rename = "stress-many-objects-cbor")]
fn stress_many_objects_cbor(
    values: Vec<StressObjectCbor>,
) -> Result<Vec<StressObjectCbor>, String> {
    Ok(values)
}
