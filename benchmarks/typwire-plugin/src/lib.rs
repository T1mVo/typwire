typwire::initiate_protocol!();

#[typwire::export(rename = "bench")]
fn bench(value: typwire::Any) -> Result<typwire::Any, String> {
    Ok(value)
}

#[derive(serde::Serialize, serde::Deserialize, typwire::Typwire)]
struct Object {
    x: i64,
    y: i64,
    label: String,
}

#[typwire::export(rename = "ints")]
fn ints(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}

#[typwire::export(rename = "string")]
fn string(value: String) -> Result<String, String> {
    Ok(value)
}

#[typwire::export(rename = "bytes")]
fn bytes(value: typwire::Bytes) -> Result<typwire::Bytes, String> {
    Ok(value)
}

#[typwire::export(rename = "objects")]
fn objects(value: Vec<Object>) -> Result<Vec<Object>, String> {
    Ok(value)
}

type Native = (typwire::Length, typwire::Angle, typwire::Ratio, typwire::Color);

#[typwire::export(rename = "natives")]
fn natives(value: Vec<Native>) -> Result<Vec<Native>, String> {
    Ok(value)
}

#[typwire::export(rename = "gradient")]
fn gradient(value: typwire::Gradient) -> Result<typwire::Gradient, String> {
    Ok(value)
}

#[typwire::export(rename = "object")]
fn object(value: Object) -> Result<Object, String> {
    Ok(value)
}
