pub struct Response {
    status_code: u32,
    body: String
}

impl Response {
    pub fn from_params(status_code: u32, body: &str) -> Self {
        Self {
            status_code,
            body: body.to_string()
        }
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }
}
