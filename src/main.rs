use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const TEMPLATES: &str = "templates";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // branch response
    let (status_line, file_name) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            format!("{}/hello.html", TEMPLATES),
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            format!("{}/hello.html", TEMPLATES),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            format!("{}/404.html", TEMPLATES),
        )
    };
    // read html file
    let mut file = File::open(file_name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // set response
    let response = format!("{}{}", status_line, contents);

    // response using TCP stream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
