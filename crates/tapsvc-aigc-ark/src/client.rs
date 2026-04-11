use reqwest::Client;

pub struct ArkClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl ArkClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
