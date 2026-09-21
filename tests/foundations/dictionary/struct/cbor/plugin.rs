// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[derive(Typwire)]` on a user struct.
#[derive(Serialize, Deserialize, Typwire)]
struct PointCbor {
    x: f64,
    y: f64,
    label: String,
}

#[typwire::export(rename = "roundtrip-struct-cbor")]
fn roundtrip_struct_cbor(value: PointCbor) -> Result<PointCbor, String> {
    Ok(value)
}
