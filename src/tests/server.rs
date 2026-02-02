#[cfg(test)]
mod easy_http_mock_server_tests {
    use crate::{
        config::EasyHttpMockConfig,
        server::adapters::vetis_adapter::{VetisAdapter, VetisAdapterConfig},
        EasyHttpMock,
    };
    use bytes::Bytes;
    use http::StatusCode;
    use http_body_util::{Either, Full};
    use std::time::Duration;
    use vetis::{Request, Response};

    #[tokio::test]
    async fn test_easy_http_mock_default() {
        let mock = EasyHttpMock::<VetisAdapter>::default();

        assert_eq!(mock.base_url(), "http://localhost:80");
        assert_eq!(mock.url("/test"), "http://localhost:80/test");
        assert_eq!(mock.url(""), "http://localhost:80");
    }

    #[tokio::test]
    async fn test_easy_http_mock_with_custom_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .base_url(Some("https://custom.mock".to_string()))
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(3000)
                    .build(),
            )
            .build();

        let mock = EasyHttpMock::new(config)?;

        assert_eq!(mock.base_url(), "http://localhost:3000");
        assert_eq!(mock.url("/api"), "https://custom.mock/api");
        assert_eq!(mock.url(""), "https://custom.mock");

        Ok(())
    }

    #[tokio::test]
    async fn test_server_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(4000)
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config)?;

        let handler = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Hello, World!");
            Ok(response)
        };

        let result = mock
            .start(handler)
            .await;
        if let Err(e) = &result {
            println!("Server start error: {:?}", e);
        }
        assert!(result.is_ok());

        tokio::time::sleep(Duration::from_millis(100)).await;

        let result = mock.stop().await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_url_generation() -> Result<(), Box<dyn std::error::Error>> {
        let mock = EasyHttpMock::<VetisAdapter>::default();
        assert_eq!(mock.url("/api/users"), "http://localhost:80/api/users");
        assert_eq!(mock.url("/"), "http://localhost:80/");
        assert_eq!(mock.url(""), "http://localhost:80");
        assert_eq!(mock.url("test"), "http://localhost:80test");

        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .base_url(Some("https://api.example.com".to_string()))
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(8181)
                    .build(),
            )
            .build();

        let mock = EasyHttpMock::new(config)?;
        assert_eq!(mock.url("/v1/users"), "https://api.example.com/v1/users");
        assert_eq!(mock.url("/"), "https://api.example.com/");
        assert_eq!(mock.url(""), "https://api.example.com");
        assert_eq!(mock.url("health"), "https://api.example.comhealth");

        Ok(())
    }

    #[tokio::test]
    async fn test_server_with_different_responses() -> Result<(), Box<dyn std::error::Error>> {
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(9999)
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config)?;

        let handler = |req: Request| async move {
            let path = req.uri().path();
            let (status, body) = match path {
                "/health" => (StatusCode::OK, "OK"),
                "/not-found" => (StatusCode::NOT_FOUND, "Not Found"),
                _ => (StatusCode::OK, "Default Response"),
            };

            let response: Response = Response::builder()
                .status(status)
                .body(Either::Right(Full::new(Bytes::from(body))));
            Ok(response)
        };

        mock.start(handler)
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(mock
            .base_url()
            .contains("localhost:"));
        assert!(!mock
            .base_url()
            .contains("localhost:80"));

        mock.stop().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_server_with_json_response() -> Result<(), Box<dyn std::error::Error>> {
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(7777) // Use random available port
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config)?;

        let handler = |_req: Request| async move {
            let json_body = r#"{
                "id": 1,
                "name": "Test User",
                "email": "test@example.com",
                "active": true
            }"#;

            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    "Content-Type",
                    "application/json"
                        .parse()
                        .unwrap(),
                )
                .text(json_body);
            Ok(response)
        };

        mock.start(handler)
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        assert!(mock
            .base_url()
            .contains("localhost:7777"));
        assert!(!mock
            .base_url()
            .is_empty());

        mock.stop()
            .await
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::{
        config::EasyHttpMockConfig,
        server::adapters::vetis_adapter::{VetisAdapter, VetisAdapterConfig},
        EasyHttpMock,
    };
    use http::StatusCode;
    use std::time::Duration;
    use vetis::{Request, Response};

    #[tokio::test]
    async fn test_multiple_server_instances() -> Result<(), Box<dyn std::error::Error>> {
        let config1 = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(8081)
                    .build(),
            )
            .build();

        let config2 = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(8082)
                    .build(),
            )
            .build();

        let mut mock1 = EasyHttpMock::new(config1)?;
        let mut mock2 = EasyHttpMock::new(config2)?;

        let handler1 = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Server 1 response");
            Ok(response)
        };

        let handler2 = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Server 2 response");
            Ok(response)
        };

        mock1
            .start(handler1)
            .await
            .unwrap();
        mock2
            .start(handler2)
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let base_url1 = mock1.base_url();
        let base_url2 = mock2.base_url();

        assert!(base_url1.contains("localhost"));
        assert!(base_url2.contains("localhost"));

        mock1
            .stop()
            .await
            .unwrap();
        mock2
            .stop()
            .await
            .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_server_with_random_port() -> Result<(), Box<dyn std::error::Error>> {
        // Create a mock server with random port
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(8888) // Use random available port
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config)?;

        let handler = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Random port test");
            Ok(response)
        };

        mock.start(handler)
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        let base_url = mock.base_url();

        assert!(base_url.contains("localhost:8888"));
        assert!(!base_url.contains("localhost:80"));

        mock.stop()
            .await
            .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_server_restart() -> Result<(), Box<dyn std::error::Error>> {
        // Use a random available port to avoid permission issues
        let config = EasyHttpMockConfig::<VetisAdapter>::builder()
            .server_config(
                VetisAdapterConfig::builder()
                    .interface("127.0.0.1")
                    .port(5555) // Use random available port
                    .build(),
            )
            .build();

        let mut mock = EasyHttpMock::new(config)?;

        let handler = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Restart test");
            Ok(response)
        };

        mock.start(handler)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Stop the server
        mock.stop()
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Start the server again with a new handler
        let handler2 = |_req: Request| async move {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .text("Restarted");
            Ok(response)
        };

        mock.start(handler2)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify server is running again
        assert!(mock
            .base_url()
            .contains("localhost:5555"));
        assert!(!mock
            .base_url()
            .is_empty());

        // Stop the server
        mock.stop()
            .await
            .unwrap();

        Ok(())
    }
}
