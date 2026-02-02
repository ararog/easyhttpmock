use http::StatusCode;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    server::{adapters::vetis_adapter::VetisServerAdapter, PortGenerator},
    EasyHttpMock,
};

use deboa::{cert::ContentEncoding, request::DeboaRequest, Client};

use vetis::{
    Response, config::{SecurityConfig, ServerConfig, VirtualHostConfig}
};

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tls_config = SecurityConfig::builder()
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build();

    let listener_config = vetis::config::ListenerConfig::builder()
        .port(8080)
        .interface("0.0.0.0")
        .build();

    let host_config = VirtualHostConfig::builder()
        .security(tls_config)
        .hostname("localhost")
        .port(8080)
        .build();

    let vetis_config = ServerConfig::builder()
        .add_listener(listener_config)
        .build();

    let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
        .server_config(vetis_config)
        .build();

    let mut server = EasyHttpMock::new(config);
    #[allow(unused_must_use)]
    let result = server
        .start(|_| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello World"))
        })
        .await;

    result.unwrap_or_else(|err| {
        panic!("Failed to start mock server: {}", err);
    });

    let client = Client::builder()
        .certificate(deboa::cert::Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = DeboaRequest::get(server.url("/anything").await)?.build()?;

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
