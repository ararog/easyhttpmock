use crate::vetis_adapter::{VetisAdapter, VetisAdapterConfig};
use deboa::{
    cert::{Certificate as _, ContentEncoding},
    request::get,
    HttpClient, HttpVersion,
};
use deboa_compio::cert::DeboaCertificate;
use easyhttpmock::{
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::PortGenerator,
    EasyHttpMock,
};
use http::StatusCode;
use std::error::Error;
use vetis_compio::Protocol;

const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

pub(crate) const fn deboa_default_protocol() -> HttpVersion {
    #[cfg(feature = "http1")]
    return HttpVersion::Http1;
    #[cfg(feature = "http2")]
    return HttpVersion::Http2;
    #[cfg(feature = "http3")]
    return HttpVersion::Http3;
}

pub(crate) const fn vetis_default_protocol() -> Protocol {
    #[cfg(feature = "http1")]
    return Protocol::Http1;
    #[cfg(feature = "http2")]
    return Protocol::Http2;
    #[cfg(feature = "http3")]
    return Protocol::Http3;
}

#[compio::test]
async fn test_mock_request() -> Result<(), Box<dyn Error>> {
    let server_cert = SERVER_CERT;
    let server_key = SERVER_KEY;

    let vetis_adapter_config = VetisAdapterConfig::builder()
        .protocol(vetis_default_protocol())
        .with_random_port()
        .cert(server_cert.to_vec())
        .key(server_key.to_vec())
        .ca(CA_CERT.to_vec())
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let Ok(mut server) = EasyHttpMock::new(config) else {
        panic!("Failed to create mock server");
    };

    let mock = Mock::of(
        given(path("/test").and(method("GET"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"teste"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let client = deboa_compio::Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .protocol(deboa_default_protocol())
        .build();

    let request = get(server.url("/test"))?.build()?;
    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}
