use crate::Matcher;

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
    use super::be_err;
    use super::be_ok;
    use super::ErrMatcher;
    use super::OkMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn ok_matcher_should_match_if_actual_is_ok() {
        assert!(OkMatcher {}.match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn ok_matcher_should_not_match_if_actual_is_err() {
        assert!(!OkMatcher {}.match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn ok_matcher_failure_messages() {
        assert_eq!(
            OkMatcher {}.failure_message(&Err::<u32, &str>("boo")),
            String::from("\tExpected:\n\t\tErr(\"boo\")\n\tto be Ok")
        );
        assert_eq!(
            OkMatcher {}.negated_failure_message(&Ok::<u32, &str>(42)),
            String::from("\tExpected:\n\t\tOk(42)\n\tnot to be Ok")
        );
    }

    #[test]
    fn be_ok_should_contruct_an_ok_matcher() {
        expect(&Ok::<u32, &str>(42)).to(be_ok())
    }

    #[test]
    fn err_matcher_should_match_if_actual_is_an_err() {
        assert!(ErrMatcher {}.match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn err_matcher_should_not_match_if_actual_is_ok() {
        assert!(!ErrMatcher {}.match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn err_matcher_failure_messages() {
        assert_eq!(
            ErrMatcher {}.failure_message(&Ok::<u32, &str>(42)),
            String::from("\tExpected:\n\t\tOk(42)\n\tto be an Err")
        );
        assert_eq!(
            ErrMatcher {}.negated_failure_message(&Err::<u32, &str>("boo")),
            String::from("\tExpected:\n\t\tErr(\"boo\")\n\tnot to be an Err")
        );
    }

    #[test]
    fn be_err_should_contruct_an_err_matcher() {
        expect(&Err::<u32, &str>("boo")).to(be_err())
    }
}
