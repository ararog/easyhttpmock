use thiserror::Error;

#[derive(Debug, Error)]
pub enum EasyHttpMockError {
    #[error("Server error: {0}")]
    Server(#[from] ServerError),
    #[error("Mock error: {0}")]
    Mock(#[from] MockError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ServerError {
    #[error("Server config error: {0}")]
    Config(String),
    #[error("Server start error: {0}")]
    Start(String),
    #[error("Server stop error: {0}")]
    Stop(String),
    #[error("Server creation error: {0}")]
    Creation(String),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum MockError {
    #[error("Mock not found")]
    Notfound,
    #[error("Mock already exists")]
    AlreadyExists,
    #[error("Request error: {0}")]
    Request(#[from] RequestError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum RequestError {
    #[error("Request failed: {0}")]
    Failed(String),
    #[error("Invalid path, expected {0}")]
    InvalidPath(String),
    #[error("Invalid method, expected {0}")]
    InvalidMethod(String),
    #[error("Invalid query param, expected {0}")]
    InvalidQuery(String),
    #[error("Invalid body, expected {0}")]
    InvalidBody(String),
    #[error("Invalid header, expected {0}")]
    InvalidHeader(String),
}
