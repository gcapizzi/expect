pub mod collection;
pub mod option;
pub mod path;
pub mod result;
pub mod string;

use crate::{Description, Matcher};

/// Matches if `expected` is equal to the actual value.
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::equal};
/// expect(&"foo").to(equal("foo"));
/// expect(&"foo").not_to(equal("bar"));
/// ```
pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher { expected }
}

pub struct EqualMatcher<T> {
    expected: T,
}

impl<E: std::fmt::Debug, A: PartialEq<E>> Matcher<A> for EqualMatcher<E> {
    fn match_value(&self, actual: &A) -> bool {
        actual == &self.expected
    }

    fn description(&self, _actual: &A) -> Description {
        Description {
            verb: String::from("equal"),
            object: Some(format!("{:?}", self.expected)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_equals_expected() {
        assert!(equal("foo").match_value(&"foo"))
    }

    #[test]
    fn should_not_match_if_actual_does_not_equal_expected() {
        assert!(!equal("foo").match_value(&"bar"))
    }

    #[test]
    fn should_allow_comparisons_between_partialeq_values() {
        assert!(equal("foo").match_value(&String::from("foo")));
    }

    #[test]
    fn should_describe_itself() {
        let description = equal("foo").description(&"bar");
        assert_eq!(description.verb, String::from("equal"));
        assert_eq!(description.object, Some(String::from("\"foo\"")));
    }
}
