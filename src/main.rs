use std::fs;
use std::net::{TcpListener, TcpStream};

mod constants;
mod http_request;
mod http_response;

use http_request::HttpRequest;
use http_response::HttpResponse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let req = HttpRequest::from(&mut stream);
    let method = req.method();
    let uri = req.uri();

    if method == "GET" {
        // Automatically add index.html if URI is a directory, or .html otherwise
        // TODO better handling of URIs
        let path = if uri.ends_with("/") {
            format!("html{uri}index.html")
        } else if !uri.ends_with(".html") {
            format!("html{uri}.html")
        } else {
            format!("html{uri}")
        };

        if let Ok(content) = fs::read_to_string(path) {
            let mut res = HttpResponse::new(200, "OK");
            res.set_body(content);
            res.send(&mut stream);
        } else {
            let mut res = HttpResponse::new(404, "Not Found");

            let not_found_content = match fs::read_to_string("html/404.html") {
                Ok(content) => content,
                Err(_) => String::from("404 Not Found"),
            };

            res.set_body(not_found_content);
            res.send(&mut stream);
        }
    } else {
        let mut res = HttpResponse::new(405, "Method not allowed");

        let content = String::from("405 Method not allowed");
        res.set_body(content);
        res.send(&mut stream);
    }
}
