struct Service;

impl Service {
    #[typwire::export]
    fn invalid(&self, value: String) -> Result<String, String> {
        Ok(value)
    }
}

fn main() {}
