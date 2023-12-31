use env_logger::Env;
use k8s_openapi::{api::core::v1::Service, serde::Serialize, serde_json::json};
use kube::Api;
use log::{info, warn};
use once_cell::sync::Lazy;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

static mut PORT: i32 = 8084;
static mut REFRESH_INTERVAL: i32 = 30;
static mut KUBE_API_URL: Lazy<String> = Lazy::new(|| "http://localhost:8080".to_string());
static mut SERVICES_MAP: Lazy<std::collections::HashMap<String, AppService>> =
    Lazy::new(|| std::collections::HashMap::new());

#[derive(Serialize, Clone)]
struct AppService {
    name: String,
    labels: Vec<String>,
    ip: String,
    port: i32,
    url: String,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Application starting");
    load_enviroment_variables();
    {
        thread::spawn(|| loop {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(load_services());
            show_services_in_logs();
            unsafe {
                info!("Sleeping for {} seconds", REFRESH_INTERVAL);
            }
            std::thread::sleep(std::time::Duration::from_secs(unsafe {
                REFRESH_INTERVAL.try_into().unwrap()
            }));
        });
    }
    info!("Listening via HTTP on this server {}", return_server());
    let listener = TcpListener::bind(listern_address()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    info!("Connection established");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let health_request = b"GET /health HTTP/1.1\r\n";
    let services_request = b"GET /services HTTP/1.1\r\n";
    let (status_line, contents) = if buffer.starts_with(health_request) {
        ("HTTP/1.1 200 OK", "OK".to_string())
    } else if buffer.starts_with(services_request) {
        ("HTTP/1.1 200 OK", get_services_json())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404".to_string())
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    if buffer.starts_with(health_request) {
        info!("Health check connection established");
    }
}

fn get_services_json() -> String {
    let mut services: Vec<AppService> = Vec::new();
    unsafe {
        for kv in SERVICES_MAP.iter() {
            let service = kv.1;
            services.push(service.clone());
        };
    };
    let json_string = json!(services).to_string();
    json_string
}

fn show_services_in_logs() {
    unsafe {
        for kv in SERVICES_MAP.iter() {
            let service = kv.1;
            info!(
                "Service {} with labels {:?} is available at {}:{} | URL: {}:{}",
                service.name, service.labels, service.ip, service.port, service.url, service.port
            );
        };
    };
}

async fn load_services() {
    info!("Loading services");
    let client = kube::Client::try_default().await.unwrap();
    let services: Api<Service> = Api::default_namespaced(client);
    let s = services.list(&Default::default()).await.unwrap();
    info!("Found {} services", s.items.len());
    for service in s.items {
        info!("Found service {}", service.metadata.name.clone().unwrap());
        let calculated_url = format!(
            "{}.{}.svc.cluster.local",
            service.metadata.name.clone().unwrap(),
            service.metadata.namespace.clone().unwrap()
        );
        info!("Calculated URL {} ", calculated_url);
        let mut labels: Vec<String> = Vec::new();
        for (key, value) in service.metadata.labels.unwrap().iter() {
            labels.push(format!("{}={}", key, value));
        }
        let app_service = AppService {
            name: service.metadata.name.clone().unwrap(),
            labels: labels,
            ip: service.spec.clone().unwrap().cluster_ip.unwrap(),
            port: service.spec.clone().unwrap().ports.unwrap()[0].port,
            url: calculated_url,
        };
        unsafe { SERVICES_MAP.insert(
            service.metadata.name.clone().unwrap(),
            app_service
        ) };
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
            unsafe { PORT = val.parse::<i32>().unwrap() };
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
    match std::env::var("KUBE_API_URL") {
        Ok(val) => {
            info!(
                "Found KUBE_API_URL environment variable, setting KUBE_API_URL to {}",
                val
            );
            unsafe {
                KUBE_API_URL.clone_from(&val);
                info!("KUBE_API_URL is {}", KUBE_API_URL.as_str());
            };
        }
        Err(_e) => {
            unsafe {
                warn!(
                    "No KUBE_API_URL environment variable found, using default url {}",
                    KUBE_API_URL.as_str()
                )
            };
        }
    }
    match std::env::var("REFRESH_INTERVAL") {
        Ok(val) => {
            info!(
                "Found REFRESH_INTERVAL environment variable, setting REFRESH_INTERVAL to {}",
                val
            );
            unsafe { REFRESH_INTERVAL = val.parse::<i32>().unwrap() };
        }
        Err(_e) => {
            unsafe {
                warn!(
                    "No REFRESH_INTERVAL environment variable found, using default interval {}",
                    REFRESH_INTERVAL
                )
            };
        }
    }
}

fn return_server() -> String {
    format!("http://{}", listern_address())
}

fn listern_address() -> String {
    format!("0.0.0.0:{}", unsafe { PORT.to_string() })
}
