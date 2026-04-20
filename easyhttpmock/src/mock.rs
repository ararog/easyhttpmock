use std::collections::HashMap;

use bytes::Bytes;
use http::{HeaderMap, Method, StatusCode};

pub struct Mock {
    request: Request,
    count: u32,
    match_request: Option<ActualRequest>,
}

impl Mock {
    #[inline]
    pub fn of(request: Request) -> Self {
        Self { request, count: 0, match_request: None }
    }

    #[inline]
    pub fn report_call(&mut self) {
        self.count += 1;
    }

    #[inline]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[inline]
    pub fn match_with(&mut self, request: ActualRequest) {
        self.match_request = Some(request);
    }
}

pub trait StatusCodeExt {
    fn respond(self) -> RespondBuilder;
}

impl StatusCodeExt for StatusCode {
    fn respond(self) -> RespondBuilder {
        RespondBuilder { status_code: self, headers: HashMap::new() }
    }
}

/// Extension trait for HTTP methods to create requests.
/// Allows creating requests using method names as strings or Method enum values.
///
/// # Examples
/// ``` compile_fail
/// use http::Method;
/// use deboa::request::MethodExt;
///
/// // Using Method enum
/// let request = Method::GET.has();
///
/// // Using string
/// let request = "GET".has();
/// ```
pub trait MethodExt {
    fn has(self) -> RequestBuilder;
}

impl MethodExt for Method {
    #[inline]
    fn has(self) -> RequestBuilder {
        match self {
            Method::GET => Request::builder(self),
            Method::POST => Request::builder(self),
            Method::PUT => Request::builder(self),
            Method::DELETE => Request::builder(self),
            Method::PATCH => Request::builder(self),
            Method::HEAD => Request::builder(self),
            Method::OPTIONS => Request::builder(self),
            _ => panic!("Method not supported"),
        }
    }
}

impl MethodExt for &str {
    #[inline]
    fn has(self) -> RequestBuilder {
        match self {
            "GET" | "get" => Request::builder(Method::GET),
            "POST" | "post" => Request::builder(Method::POST),
            "PUT" | "put" => Request::builder(Method::PUT),
            "DELETE" | "delete" => Request::builder(Method::DELETE),
            "PATCH" | "patch" => Request::builder(Method::PATCH),
            "HEAD" | "head" => Request::builder(Method::HEAD),
            "OPTIONS" | "options" => Request::builder(Method::OPTIONS),
            _ => panic!("Method not supported"),
        }
    }
}

pub struct ActualRequestBuilder {
    path: String,
    method: Method,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: String,
}

impl ActualRequestBuilder {
    #[inline]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    #[inline]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[inline]
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    #[inline]
    pub fn query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = query_params;
        self
    }

    #[inline]
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    #[inline]
    pub fn build(self) -> ActualRequest {
        ActualRequest {
            path: self.path,
            method: self.method,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
        }
    }
}

pub struct ActualRequest {
    path: String,
    method: Method,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: String,
}

impl ActualRequest {
    #[inline]
    pub fn builder() -> ActualRequestBuilder {
        ActualRequestBuilder {
            path: String::new(),
            method: Method::GET,
            headers: HeaderMap::new(),
            query_params: HashMap::new(),
            body: String::new(),
        }
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    #[inline]
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }
}

pub struct RequestBuilder {
    path: String,
    method: Method,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: Bytes,
}

impl RequestBuilder {
    #[inline]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    #[inline]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    #[inline]
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    #[inline]
    pub fn query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = query_params;
        self
    }

    #[inline]
    pub fn body(mut self, body: &[u8]) -> Self {
        self.body = Bytes::from(body.to_vec());
        self
    }

    #[inline]
    pub fn will_return(self, respond: Respond) -> Request {
        Request {
            path: self.path,
            method: self.method,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
            respond,
        }
    }
}

pub struct Request {
    path: String,
    method: Method,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: Bytes,
    respond: Respond,
}

impl Request {
    #[inline]
    pub fn builder(method: Method) -> RequestBuilder {
        RequestBuilder {
            path: String::new(),
            method,
            headers: HeaderMap::new(),
            query_params: HashMap::new(),
            body: Bytes::new(),
        }
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    #[inline]
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    #[inline]
    pub fn respond(&self) -> &Respond {
        &self.respond
    }
}

pub struct RespondBuilder {
    status_code: StatusCode,
    headers: HashMap<String, String>,
}

impl RespondBuilder {
    #[inline]
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    #[inline]
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    #[inline]
    pub fn with_headers(mut self, entries: &[(&str, &str)]) -> Self {
        for (key, value) in entries {
            self.headers
                .insert(key.to_string(), value.to_string());
        }
        self
    }

    #[inline]
    pub fn empty(self) -> Respond {
        self.no_body()
    }

    #[inline]
    pub fn no_body(self) -> Respond {
        Respond { status_code: self.status_code, headers: self.headers, body: Bytes::new() }
    }

    #[inline]
    pub fn with_body(self, body: &[u8]) -> Respond {
        Respond {
            status_code: self.status_code,
            headers: self.headers,
            body: Bytes::from(body.to_vec()),
        }
    }
}

pub struct Respond {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Bytes,
}

impl Respond {
    #[inline]
    pub fn builder() -> RespondBuilder {
        RespondBuilder { status_code: StatusCode::OK, headers: HashMap::new() }
    }

    #[inline]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[inline]
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    #[inline]
    pub fn body(&self) -> Bytes {
        self.body.clone()
    }
}
