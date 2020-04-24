use crate::Matcher;

/// Matches if `actual` is a [`Result::Ok`].
///
/// [`Restul::Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::result::be_ok};
/// expect(&Ok::<&str, &str>("foo")).to(be_ok());
/// expect(&Err::<&str, &str>("bar")).not_to(be_ok());
/// ```
pub fn be_ok() -> OkMatcher {
    OkMatcher {}
}

pub struct OkMatcher {}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Matcher<Result<T, E>> for OkMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> bool {
        actual.is_ok()
    }

    fn failure_message(&self, actual: &Result<T, E>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto be Ok", actual)
    }

    fn negated_failure_message(&self, actual: &Result<T, E>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to be Ok", actual)
    }
}

/// Matches if `actual` is a [`Result::Err`].
///
/// [`Restul::Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::result::be_err};
/// expect(&Err::<&str, &str>("foo")).to(be_err());
/// expect(&Ok::<&str, &str>("bar")).not_to(be_err());
/// ```
pub fn be_err() -> ErrMatcher {
    ErrMatcher {}
}

pub struct ErrMatcher {}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Matcher<Result<T, E>> for ErrMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> bool {
        actual.is_err()
    }

    fn failure_message(&self, actual: &Result<T, E>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto be an Err", actual)
    }

    fn negated_failure_message(&self, actual: &Result<T, E>) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to be an Err", actual)
    }
}

#[cfg(test)]
mod tests {
    use super::{be_err, be_ok};
    use crate::Matcher;

    #[test]
    fn ok_matcher_should_match_if_actual_is_ok() {
        assert!(be_ok().match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn ok_matcher_should_not_match_if_actual_is_err() {
        assert!(!be_ok().match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn ok_matcher_failure_messages() {
        assert_eq!(
            be_ok().failure_message(&Err::<u32, &str>("boo")),
            String::from("\tExpected:\n\t\tErr(\"boo\")\n\tto be Ok")
        );
        assert_eq!(
            be_ok().negated_failure_message(&Ok::<u32, &str>(42)),
            String::from("\tExpected:\n\t\tOk(42)\n\tnot to be Ok")
        );
    }

    #[test]
    fn err_matcher_should_match_if_actual_is_an_err() {
        assert!(be_err().match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn err_matcher_should_not_match_if_actual_is_ok() {
        assert!(!be_err().match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn err_matcher_failure_messages() {
        assert_eq!(
            be_err().failure_message(&Ok::<u32, &str>(42)),
            String::from("\tExpected:\n\t\tOk(42)\n\tto be an Err")
        );
        assert_eq!(
            be_err().negated_failure_message(&Err::<u32, &str>("boo")),
            String::from("\tExpected:\n\t\tErr(\"boo\")\n\tnot to be an Err")
        );
    }
}
