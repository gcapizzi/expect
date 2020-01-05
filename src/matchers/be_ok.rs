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

#[cfg(test)]
mod tests {
    use super::be_ok;
    use super::OkMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_ok() {
        assert!(OkMatcher {}.match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn should_not_match_if_actual_is_err() {
        assert!(!OkMatcher {}.match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn failure_messages() {
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
    fn be_some_should_contruct_a_some_matcher() {
        expect(&Ok::<u32, &str>(42)).to(be_ok())
    }
}
