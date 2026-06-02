use caramelo::assertions::{Is, IsEq};

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

    config
        .server_config()
        .port()
        .is_eq(&8080);

    config
        .server_config()
        .interface()
        .is_eq(&"127.0.0.1");

    config
        .base_url()
        .is(Some("http://127.0.0.1:8080".to_string()));
}
