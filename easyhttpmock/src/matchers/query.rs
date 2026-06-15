use caramelo::Matcher;

use crate::mock::Request;

/// Creates a matcher that checks if the request query matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Query` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Panics
///
/// * `Invalid regex pattern` - If the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query;
///
/// let matcher = query(r"^/api/v1/.*$");
/// ```
pub fn query(value: &str) -> Query {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Query(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

#[derive(Clone)]
/// A matcher that checks if the request query matches a regex pattern.
///
/// # Arguments
///
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `Query` - A matcher that checks if the request query matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use easyhttpmock::matchers::query;
///
/// let matcher = query(r"^/api/v1/.*$");
/// ```
pub struct Query(regex::Regex);

impl Matcher<Request> for Query {
    fn matches(&self, value: &Request) -> bool {
        self.0
            .is_match(value.path())
    }

    fn description(&self) -> String {
        format!("query matching {:?}", self.0)
    }
}
