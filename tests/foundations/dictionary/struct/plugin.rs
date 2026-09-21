// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[derive(Typwire)]` on a user struct.
#[derive(Serialize, Deserialize, Typwire)]
struct Point {
    x: f64,
    y: f64,
    label: String,
}

#[typwire::export]
fn roundtrip_struct(value: Point) -> Result<Point, String> {
    Ok(value)
}
