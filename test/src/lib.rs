use serde::{Deserialize, Serialize};
use typwire::{Any, Length, OneOf3, Typwire, initiate_protocol};

initiate_protocol!();

#[derive(Serialize, Deserialize, Typwire)]
#[serde(rename_all = "kebab-case")]
struct Options {
    label: String,
    scale: f64,
}

#[derive(Serialize, Deserialize, Typwire)]
struct Test {
    text: String,
    width: Length,
    metadata: Any,
}

#[typwire::export]
fn measure(
    value: OneOf3<i64, f64, String>,
    options: Options,
    fallback: Option<Length>,
    metadata: Any,
) -> Result<Test, String> {
    let value = match value {
        OneOf3::First(value) => value.to_string(),
        OneOf3::Second(value) => value.to_string(),
        OneOf3::Third(value) => value,
    };

    Ok(Test {
        text: format!("{}{}", options.label, value),
        width: fallback.unwrap_or_else(|| Length::new(12.0 * options.scale)),
        metadata,
    })
}

#[typwire::export]
fn echo(value: Any) -> Result<Any, String> {
    Ok(value)
}

#[typwire::export]
fn fail(message: String) -> Result<(), String> {
    Err(message)
}
