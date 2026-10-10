use crate::{AsyncShared, SharedTypedMatcher};
use caramelo::{
    MatchType::{self, To},
    Matcher, TypedMatcher,
};

/// Creates a matcher that matches values that satisfy all given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub fn and<T: AsyncShared + 'static>(
    matchers: Vec<SharedTypedMatcher<T>>,
) -> SharedTypedMatcher<T> {
    SharedTypedMatcher::new(And {matchers})
}

/// Matcher that combines multiple matchers with AND logic
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub struct And<T: AsyncShared> {
    matchers: Vec<SharedTypedMatcher<T>>,
}

#[cfg(feature = "multi-threaded")]
unsafe impl<T: AsyncShared> Send for And<T> {}
#[cfg(feature = "multi-threaded")]
unsafe impl<T: AsyncShared> Sync for And<T> {}

impl<T: AsyncShared> And<T> {
    /// Creates a new And matcher with the given matchers
    pub fn new(matchers: Vec<SharedTypedMatcher<T>>) -> Self {
        And { matchers }
    }
}

impl<T> Matcher<T> for And<T>
where
    T: AsyncShared,
{
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .all(|m| m.matches(value))
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" and ")
    }
}

impl<T> TypedMatcher<T> for And<T>
where
    T: AsyncShared,
{
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatcher::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
