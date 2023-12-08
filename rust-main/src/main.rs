extern crate log;
extern crate env_logger;
use env_logger::Env;
use log::info;
use std::io::{Write, Read};
use std::thread;

use std::net::TcpListener;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Application starting");
    info!("Serving message {} via HTTP on this endpoint {}", return_message(), return_endpoint());
    let listener = TcpListener::bind("0.0.0.0:8084").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("Connection established");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET /hello HTTP/1.1\r\n";
    let post = b"POST /die HTTP/1.1\r\n";
    let (status_line, contents, die) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", return_message(), false)
    } else if buffer.starts_with(post) {
        ("HTTP/1.1 200 OK", "Exiting", true)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404", false)
    };

    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    if die {
        info!("Exiting");
        std::process::exit(0);
    }
}

fn return_message() -> &'static str {
    "Hello, world!"
}

fn return_endpoint() -> String {
    format!("{}/{}", return_server(), "hello")
}

fn return_server() -> &'static str {
    "http://localhost:8084"
}