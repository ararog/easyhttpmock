use http::StatusCode;

use easyhttpmock::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    server::{
        PortGenerator,
        adapters::vetis_adapter::{VetisAdapter, VetisAdapterConfig},
    },
};

use deboa::{
    Client,
    cert::{Certificate, ContentEncoding},
    request::DeboaRequest,
};

use vetis::Response;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vetis_adapter_config = VetisAdapterConfig::builder()
        .interface("0.0.0.0")
        .with_random_port()
        .cert(Some(SERVER_CERT.to_vec()))
        .key(Some(SERVER_KEY.to_vec()))
        .ca(Some(CA_CERT.to_vec()))
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let mut server = EasyHttpMock::new(config)?;
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
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let url = server.url("/anything");
    let request = DeboaRequest::get(url)?.build()?;

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
