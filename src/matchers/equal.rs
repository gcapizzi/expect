use crate::Matcher;

pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher { expected }
}

pub struct EqualMatcher<T> {
    expected: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, actual: &T) -> bool {
        actual == &self.expected
    }

    fn failure_message(&self, actual: &T) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tto equal:\n\t\t{:?}",
            self.expected, actual
        )
    }

    fn negated_failure_message(&self, actual: &T) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tnot to equal:\n\t\t{:?}",
            self.expected, actual
        )
    }
}

#[cfg(test)]
mod tests {
    use super::equal;
    use super::EqualMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_equals_expected() {
        assert!(EqualMatcher { expected: "foo" }.match_value(&"foo"))
    }

    #[test]
    fn should_not_match_if_actual_does_not_equal_expected() {
        assert!(!EqualMatcher { expected: "foo" }.match_value(&"bar"))
    }

    #[test]
    fn failure_messages() {
        assert_eq!(
            EqualMatcher { expected: "foo" }.failure_message(&"bar"),
            String::from("\tExpected:\n\t\t\"foo\"\n\tto equal:\n\t\t\"bar\"")
        );
        assert_eq!(
            EqualMatcher { expected: "foo" }.negated_failure_message(&"foo"),
            String::from("\tExpected:\n\t\t\"foo\"\n\tnot to equal:\n\t\t\"foo\"")
        );
    }

    #[test]
    fn equal_should_construct_an_equal_matcher() {
        expect(&"foo").to(equal("foo"))
    }
}
