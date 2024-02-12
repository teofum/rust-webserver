use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use crate::constants;

pub struct HttpRequest {
    method: String,
    uri: String,
}

impl HttpRequest {
    pub fn from(mut stream: &mut TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        let request: Vec<_> = request_line.split(" ").collect();

        match request[..] {
            [method, uri, constants::HTTP_VER] => HttpRequest {
                method: method.to_string(),
                uri: uri.to_string(),
            },
            _ => panic!("Bad HTTP request header"),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}
