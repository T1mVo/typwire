#[typwire::export]
async fn invalid(value: String) -> Result<String, String> {
    Ok(value)
}

fn main() {}
