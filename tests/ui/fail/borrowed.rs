#[typwire::export]
fn invalid(value: &str) -> Result<String, String> {
    Ok(value.to_owned())
}

fn main() {}
