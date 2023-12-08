extern crate log;
extern crate env_logger;
use env_logger::Env;
use log::info;
use std::io::Write;
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
    let status_line = "HTTP/1.1 200 OK";
    let contents = return_message();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
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