use std::collections::HashMap;

use http::{HeaderMap, StatusCode};

pub struct WhenBuilder {
    path: String,
    method: String,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: String,
}

impl WhenBuilder {
    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn method(mut self, method: String) -> Self {
        self.method = method;
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = query_params;
        self
    }

    pub fn body(mut self, body: String) -> When {
        self.body = body;
        When {
            path: self.path,
            method: self.method,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
            errors: Vec::new(),
        }
    }
}

pub struct When {
    path: String,
    method: String,
    headers: HeaderMap,
    query_params: HashMap<String, String>,
    body: String,
    errors: Vec<String>,
}

impl When {
    pub fn builder() -> WhenBuilder {
        WhenBuilder {
            path: String::new(),
            method: String::new(),
            headers: HeaderMap::new(),
            query_params: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn path(mut self, path: String) -> Self {
        if !path.eq(&self.path) {
            self.errors
                .push(format!("No matching path, expected {}", self.path));
        }
        self
    }

    pub fn method(mut self, method: String) -> Self {
        if !method.eq(&self.method) {
            self.errors
                .push(format!("No matching method, expected {}", self.method));
        }
        self
    }

    pub fn query_param(mut self, key: String, value: String) -> Self {
        if !self
            .query_params
            .contains_key(&key)
        {
            self.errors
                .push(format!("No matching query param, expected {}", key));
        } else {
            let existing_value = self
                .query_params
                .get(&key)
                .unwrap();
            if !value.eq(existing_value) {
                self.errors
                    .push(format!("No matching query param value, expected {}", key));
            }
        }
        self
    }

    pub fn body(mut self, body: String) -> Self {
        if !body.eq(&self.body) {
            self.errors
                .push(format!("No matching body, expected {}", self.body));
        }
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        if !self
            .headers
            .contains_key(&key)
        {
            self.errors
                .push(format!("No matching header, expected {}", key));
        } else {
            let existing_value = self
                .headers
                .get(&key)
                .unwrap();
            if !value.eq(existing_value) {
                self.errors
                    .push(format!("No matching header value, expected {}", key));
            }
        }
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn then(self) -> ThenBuilder {
        ThenBuilder {
            status_code: StatusCode::OK,
            header: (String::new(), String::new()),
            body: String::new(),
            errors: Vec::new(),
        }
    }
}

pub struct ThenBuilder {
    status_code: StatusCode,
    header: (String, String),
    body: String,
    errors: Vec<String>,
}

impl ThenBuilder {
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.header = (key, value);
        self
    }

    pub fn with_body(mut self, body: String) -> Then {
        self.body = body;

        Then {
            status_code: self.status_code,
            header: self.header,
            body: self.body,
            errors: self.errors,
        }
    }
}

pub struct Then {
    status_code: StatusCode,
    header: (String, String),
    body: String,
    errors: Vec<String>,
}

impl Then {
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn header(&self) -> (String, String) {
        self.header.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
}
