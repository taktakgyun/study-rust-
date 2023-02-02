use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const TEMPLATES: &str = "templates";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        // read html file
        let file_name = format!("{}/hello.html", TEMPLATES);
        let mut file = File::open(file_name).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // set response
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        // response using TCP stream
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // read html file
        let file_name = format!("{}/404.html", TEMPLATES);
        let mut file = File::open(file_name).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // set response
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let response = format!("{}{}", status_line, contents);

        // response using TCP stream
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
