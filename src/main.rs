use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod http_response;
mod constants;

use http_response::HttpResponse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let mut res = HttpResponse::new(200, "OK");

        let content = fs::read_to_string("html/index.html").unwrap();
        res.set_body(content);

        stream.write_all(res.to_string().as_bytes()).unwrap();
    } else {
        let mut res = HttpResponse::new(404, "Not found");

        let content = fs::read_to_string("html/404.html").unwrap();
        res.set_body(content);

        stream.write_all(res.to_string().as_bytes()).unwrap();
    }
}
