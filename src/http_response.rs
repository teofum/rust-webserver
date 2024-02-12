use std::collections::HashMap;
use std::fmt;

use crate::constants;

pub struct HttpResponse {
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status: u16, status_text: &str) -> Self {
        HttpResponse {
            status,
            status_text: status_text.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_header(&mut self, name: &str, content: &str) {
        self.headers.insert(name.to_string(), content.to_string());
    }

    pub fn remove_header(&mut self, name: &str) {
        self.headers.remove(name);
    }

    pub fn set_body(&mut self, body: String) {
        let length = body.len();

        self.set_header("Content-Length", &length.to_string());
        self.body = Some(body);
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_line = format!(
            "{} {} {}",
            constants::HTTP_VER,
            self.status,
            self.status_text
        );

        let headers: Vec<String> = self
            .headers
            .iter()
            .map(|(key, value)| format!("{key} {value}\r\n"))
            .collect();

        let body = match &self.body {
            Some(str) => str,
            None => "",
        };

        write!(f, "{status_line}\r\n{}\r\n{body}", headers.join(""))
    }
}
