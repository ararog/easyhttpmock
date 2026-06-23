use easyhttpmock_vetis_tokio::{
    EasyHttpMock,
    config::EasyHttpMockConfig,
    matchers::{method, path},
    mock::{AsyncMatcherExt, Mock, StatusCodeExt, given},
    vetis_adapter::VetisAdapter,
};
use http::{Method, StatusCode};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = EasyHttpMockConfig::<VetisAdapter>::default();
    let server = EasyHttpMock::new(config);
    let mut server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    let mock = Mock::of(
        given(method(Method::GET).and(path("/test"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"teste"),
        ),
    );

    server.register_mock(mock).await?;

    Ok(())
}
