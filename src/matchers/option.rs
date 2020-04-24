use crate::Matcher;

/// Matches if `actual` is an [`Option::Some`].
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::option::be_some};
/// expect(&Some("foo")).to(be_some());
/// expect(&None::<&str>).not_to(be_some());
/// ```
pub fn be_some() -> SomeMatcher {
    SomeMatcher {}
}

pub struct SomeMatcher {}

impl<T: std::fmt::Debug> Matcher<Option<T>> for SomeMatcher {
    fn match_value(&self, actual: &Option<T>) -> bool {
        actual.is_some()
    }

    fn failure_message(&self, actual: &Option<T>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto be a Some", actual)
    }

    fn negated_failure_message(&self, actual: &Option<T>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to be a Some", actual)
    }
}

/// Matches if `actual` is an [`Option::None`].
///
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::option::be_none};
/// expect(&None::<&str>).to(be_none());
/// expect(&Some("foo")).not_to(be_none());
/// ```
pub fn be_none() -> NoneMatcher {
    NoneMatcher {}
}

pub struct NoneMatcher {}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Matcher<Option<T>> for NoneMatcher {
    fn match_value(&self, actual: &Option<T>) -> bool {
        actual.is_none()
    }

    fn failure_message(&self, actual: &Option<T>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto be None", actual)
    }

    fn negated_failure_message(&self, actual: &Option<T>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to be None", actual)
    }
}

#[cfg(test)]
mod tests {
    use super::{be_none, be_some};
    use crate::Matcher;

    #[test]
    fn some_matcher_should_match_if_actual_is_some() {
        assert!(be_some().match_value(&Some("foo")))
    }

    #[test]
    fn some_matcher_should_not_match_if_actual_is_none() {
        assert!(!be_some().match_value(&None::<u32>))
    }

    #[test]
    fn some_matcher_failure_messages() {
        assert_eq!(
            be_some().failure_message(&None::<&str>),
            String::from("\tExpected:\n\t\tNone\n\tto be a Some")
        );
        assert_eq!(
            be_some().negated_failure_message(&Some("foo")),
            String::from("\tExpected:\n\t\tSome(\"foo\")\n\tnot to be a Some")
        );
    }

    #[test]
    fn none_matcher_should_match_if_actual_is_none() {
        assert!(be_none().match_value(&None::<&str>))
    }

    #[test]
    fn none_matcher_should_not_match_if_actual_is_some() {
        assert!(!be_none().match_value(&Some("thing")))
    }

    #[test]
    fn none_matcher_failure_messages() {
        assert_eq!(
            be_none().failure_message(&Some("foo")),
            String::from("\tExpected:\n\t\tSome(\"foo\")\n\tto be None")
        );
        assert_eq!(
            be_none().negated_failure_message(&None::<&str>),
            String::from("\tExpected:\n\t\tNone\n\tnot to be None")
        );
    }
}
