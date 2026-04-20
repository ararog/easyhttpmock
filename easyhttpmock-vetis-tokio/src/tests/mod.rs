use std::error::Error;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    mock::{MethodExt, Mock, StatusCodeExt},
    server::PortGenerator,
    EasyHttpMock,
};
use http::{Method, StatusCode};

use crate::vetis_adapter::{VetisAdapter, VetisAdapterConfig};
use vetis_tokio::Protocol;

const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");

const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

#[tokio::test]
async fn test_mock_request() -> Result<(), Box<dyn Error>> {
    let server_cert = SERVER_CERT;
    let server_key = SERVER_KEY;

    let vetis_adapter_config = VetisAdapterConfig::builder()
        .hostname(Some("localhost".to_string()))
        .interface("0.0.0.0")
        .protocol(Protocol::Http2)
        .with_random_port()
        .cert(Some(server_cert.to_vec()))
        .key(Some(server_key.to_vec()))
        .ca(Some(CA_CERT.to_vec()))
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let server = EasyHttpMock::new(config);
    let mut server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    let mock = Mock::of(
        Method::GET
            .has()
            .path("/test")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"teste"),
            ),
    );

    server.register_mock(mock);

    Ok(())
}
