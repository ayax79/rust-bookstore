use rust_eureka::EurekaClient;
use rust_eureka::request::{RegisterRequest, Instance, Status, DataCenterInfo, DcName};
use settings::Settings;
use network::SocketInfo;
use serde_json::Map;
use tokio_core::reactor::Handle;
use rust_eureka::errors::EurekaClientError;
use futures::Future;

const APPLICATION_ID: &'static str = "rust-bookstore";

pub struct EurekaHandler<'a> {
    client: Option<EurekaClient<'a>>,
    registration: RegisterRequest,
}

impl<'a> EurekaHandler<'a> {
    pub fn new(handle: &'a Handle, settings: Settings, socket_info: &SocketInfo) -> Self {

        let client = settings.clone().eureka_url.map(|url| {
            EurekaClient::new(handle, APPLICATION_ID, url.as_ref())
        });
        let registration = build_registration(&settings, socket_info);
        EurekaHandler {
            client,
            registration,
        }
    }

    pub fn register(&self) -> Option<Box<Future<Item=(), Error=EurekaClientError>>> {
        self.client
            .as_ref()
            .map(|ref client| client.register(APPLICATION_ID, &self.registration))
    }
}


fn build_registration(settings: &Settings, socket_info: &SocketInfo) -> RegisterRequest {
    let health_url = format!("{}:/health", socket_info.base_url());
    let host_name = settings.hostname.to_owned().unwrap_or("localhost".to_string());

    RegisterRequest {
        instance: Instance {
            host_name,
            app: APPLICATION_ID.to_string(),
            ip_addr: socket_info.ip_address.to_owned(),
            vip_address: socket_info.ip_address.to_owned(),
            secure_vip_address: socket_info.ip_address.to_owned(),
            status: Status::Up,
            port: Some(socket_info.port),
            secure_port: Some(socket_info.port),
            homepage_url: health_url.to_owned(),
            status_page_url: health_url.to_owned(),
            health_check_url: health_url.to_owned(),
            data_center_info: DataCenterInfo {
                name: DcName::Amazon,
                metadata: None,
            },
            lease_info: None,
            metadata: Map::new(),
        }
    }
}


