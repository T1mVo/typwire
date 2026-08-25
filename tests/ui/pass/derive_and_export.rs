use serde::{Deserialize, Serialize};
use typwire::{Typwire, initiate_protocol};

initiate_protocol!();

#[derive(Serialize, Deserialize, Typwire)]
struct Wrapper<T> {
    value: T,
}

#[typwire::export]
fn echo(value: Wrapper<String>) -> Result<Wrapper<String>, &'static str> {
    Ok(value)
}

fn main() {}
