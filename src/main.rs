use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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
        let status = "HTTP/1.1 200 OK";
        let content = fs::read_to_string("html/index.html").unwrap();
        let len = content.len();

        let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("html/404.html").unwrap();
        let len = content.len();

        let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
