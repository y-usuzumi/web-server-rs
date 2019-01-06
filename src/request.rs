#[derive(Debug)]
pub struct Request {
    uri: String,
    method: String,
    http_version: String,
    header: String,
    body: String,
}

impl Request {
    pub fn new() -> Self {
        Self {
            uri: String::default(),
            method: String::default(),
            http_version: String::default(),
            header: String::default(),
            body: String::default(),
        }
    }

    pub fn with_params(
        uri: &str,
        method: &str,
        http_version: &str,
        header: &str,
        body: &str
        ) -> Self {
        Self {
            uri: uri.to_string(),
            method: method.to_string(),
            http_version: http_version.to_string(),
            header: header.to_string(),
            body: body.to_string(),
        }
    }

    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }

    pub fn set_method(&mut self, method: &str) {
        self.method = method.to_string();
    }

    pub fn set_http_version(&mut self, http_version: &str) {
        self.http_version = http_version.to_string();
    }

    pub fn uri(&self) -> String {
        self.uri.clone()
    }

    pub fn method(&self) -> String {
        self.method.clone()
    }
}
