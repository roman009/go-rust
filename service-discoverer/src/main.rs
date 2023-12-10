// this application will be used to discover services in the cluster
// it will call the kube api to get the list of services and keep a local cache, which
// will be refreshed every 30 seconds
// the application will also expose a http endpoint that will return the list of services
// in the cluster that can be filtered by tags
use env_logger::Env;
use k8s_openapi::api::core::v1::Service;
use kube::Api;
use log::{info, warn};
use once_cell::sync::Lazy;

static mut PORT: i32 = 8084;
static mut KUBE_API_URL: Lazy<String> = Lazy::new(|| "http://localhost:8080".to_string());
static mut SERVICES: Vec<AppService> = Vec::new();

struct AppService {
    name: String,
    tags: Vec<String>,
    ip: String,
    port: i32
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Application starting");
    load_enviroment_variables();
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(load_services());
    }
    show_services();
    
    info!("Hello, world!");
}

fn show_services() {
    unsafe {
        for service in SERVICES.iter() {
            info!("Service {} with tags {:?} is available at {}:{}", service.name, service.tags, service.ip, service.port);
        }
    }
}

async fn load_services() {
    info!("Loading services");
    let client = kube::Client::try_default().await.unwrap();
    let services: Api<Service> = Api::default_namespaced(client);
    let s = services.list(&Default::default()).await.unwrap();
    info!("Found {} services", s.items.len());
    for service in s.items {
        info!("Found service {}", service.metadata.name.clone().unwrap());
        let mut tags: Vec<String> = Vec::new();
        for (key, value) in service.metadata.labels.unwrap().iter() {
            tags.push(format!("{}={}", key, value));
        }
        let app_service = AppService {
            name: service.metadata.name.clone().unwrap(),
            tags: tags,
            ip: service.spec.clone().unwrap().cluster_ip.unwrap(),
            port: service.spec.clone().unwrap().ports.unwrap()[0].port
        };
        unsafe { SERVICES.push(app_service) };
    }
}

fn load_enviroment_variables() {
    info!("Loading environment variables");
    match std::env::var("LISTENING_PORT") {
        Ok(val) => {
            info!("Found LISTENING_PORT environment variable, setting PORT to {}", val);
            unsafe { PORT = val.parse::<i32>().unwrap() };
        },
        Err(_e) => {
            unsafe { warn!("No LISTENING_PORT environment variable found, using default port {}", PORT) };
        }
    }
    match std::env::var("KUBE_API_URL") {
        Ok(val) => {
            info!("Found KUBE_API_URL environment variable, setting KUBE_API_URL to {}", val);
            unsafe { 
                KUBE_API_URL.clone_from(&val);
                info!("KUBE_API_URL is {}", KUBE_API_URL.as_str());
            };
        },
        Err(_e) => {
            unsafe { warn!("No KUBE_API_URL environment variable found, using default url {}", KUBE_API_URL.as_str()) };
        }
    }
}