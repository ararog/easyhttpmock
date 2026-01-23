use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::Response;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    server::{adapters::vetis_adapter::VetisServerAdapter, PortGenerator},
    EasyHttpMock,
};

use deboa::{cert::ContentEncoding, request::DeboaRequest, Client};

use vetis::{
    server::{
        config::{SecurityConfig, ServerConfig},
    },
};

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = SecurityConfig::builder()
        .cert(SERVER_CERT.to_vec())
        .key(SERVER_KEY.to_vec())
        .build();

    let vetis_config = ServerConfig::builder()
        .security(tls_config)
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
        .server_config(vetis_config)
        .build();

    let mut server = EasyHttpMock::new(config);
    #[allow(unused_must_use)]
    let result = server
        .start(|_| async move {
            Ok(Response::new(Full::new(Bytes::from("Hello World"))))
        })
        .await;

    result.unwrap_or_else(|err| {
        panic!("Failed to start mock server: {}", err);
    });

    let client = Client::builder()
        .certificate(deboa::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = DeboaRequest::get(server.url("/anything"))?.build()?;

    let response = client
        .execute(request)
        .await?;

    if response.status() == StatusCode::OK {
        println!("Request executed successfully");
    }

    server
        .stop()
        .await?;
    
    Ok(())
}
