/// Create a reqwest ClientBuilder.
pub fn create_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder_creation() {
        let builder = create_client_builder();
        let _client = builder.build();
    }
}
