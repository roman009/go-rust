extern crate env_logger;
extern crate log;
use env_logger::Env;
use log::{info, warn};
use std::io::{Read, Write};
use std::thread;

use std::net::TcpListener;

static mut PORT: i16 = 8084;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Application starting");
    load_enviroment_variables();
    info!(
        "Serving message {} via HTTP on this endpoint {}",
        return_message(),
        return_endpoint()
    );
    let listener = TcpListener::bind(listern_address()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("Connection established");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn load_enviroment_variables() {
    info!("Loading environment variables");
    match std::env::var("LISTENING_PORT") {
        Ok(val) => {
            info!(
                "Found LISTENING_PORT environment variable, setting PORT to {}",
                val
            );
            unsafe { PORT = val.parse::<i16>().unwrap() };
        }
        Err(_e) => {
            unsafe {
                warn!(
                    "No LISTENING_PORT environment variable found, using default port {}",
                    PORT
                )
            };
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET /hello HTTP/1.1\r\n";
    let post = b"POST /die HTTP/1.1\r\n";
    let health = b"GET /health HTTP/1.1\r\n";
    let (status_line, contents, die) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", return_message(), false)
    } else if buffer.starts_with(post) {
        ("HTTP/1.1 200 OK", "Exiting", true)
    } else if buffer.starts_with(health) {
        ("HTTP/1.1 200 OK", "OK", false)
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404", false)
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    if buffer.starts_with(health) {
        info!("Health check connection established");
    }
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

fn return_server() -> String {
    format!("http://{}", listern_address())
}

fn listern_address() -> String {
    format!("0.0.0.0:{}", unsafe { PORT.to_string() })
}
