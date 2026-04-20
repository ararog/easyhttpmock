use crate::{
    config::EasyHttpMockConfig,
    tests::server::{TestServer, TestServerConfig},
};

#[test]
fn test_config() {
    let config = EasyHttpMockConfig::<TestServer>::builder()
        .base_url(Some("http://127.0.0.1:8080".to_string()))
        .server_config(TestServerConfig::default())
        .build();

    assert_eq!(
        config
            .server_config()
            .port(),
        8080
    );
    assert_eq!(
        config
            .server_config()
            .interface(),
        "127.0.0.1"
    );

    assert_eq!(
        config.base_url(),
        &Some("http://127.0.0.1:8080".to_string()),
        "base url should be http://127.0.0.1:8080"
    );
}
