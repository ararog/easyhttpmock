use thiserror::Error;

/// EasyHttpMock error types
#[derive(Debug, Error)]
pub enum EasyHttpMockError {
    /// Server error
    #[error("Server error: {0}")]
    Server(#[from] ServerError),
    /// Mock error
    #[error("Mock error: {0}")]
    Mock(#[from] MockError),
}

/// Server related errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ServerError {
    /// Server config error
    #[error("Server config error: {0}")]
    Config(String),
    /// Server start error
    #[error("Server start error: {0}")]
    Start(String),
    /// Server stop error
    #[error("Server stop error: {0}")]
    Stop(String),
    /// Server creation error
    #[error("Server creation error: {0}")]
    Creation(String),
}

/// Mock related errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum MockError {
    /// Mock not found
    #[error("Mock not found")]
    Notfound,
    #[error("Mock already exists")]
    /// Mock already exists
    AlreadyExists,
    /// Request error
    #[error("Request error: {0}")]
    Request(#[from] RequestError),
}

/// Request related errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum RequestError {
    /// Request failed
    #[error("Request failed: {0}")]
    Failed(String),
    /// Invalid path
    #[error("Invalid path, expected {0}")]
    InvalidPath(String),
    /// Invalid method
    #[error("Invalid method, expected {0}")]
    InvalidMethod(String),
    /// Invalid query param
    #[error("Invalid query param, expected {0}")]
    InvalidQuery(String),
    /// Invalid body
    #[error("Invalid body, expected {0}")]
    InvalidBody(String),
    /// Invalid header
    #[error("Invalid header, expected {0}")]
    InvalidHeader(String),
}
