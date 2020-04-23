use crate::Matcher;

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
    use super::be_none;
    use super::be_some;
    use super::NoneMatcher;
    use super::SomeMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn some_matcher_should_match_if_actual_is_some() {
        assert!(SomeMatcher {}.match_value(&Some("foo")))
    }

    #[test]
    fn some_matcher_should_not_match_if_actual_is_none() {
        assert!(!SomeMatcher {}.match_value(&None::<u32>))
    }

    #[test]
    fn some_matcher_failure_messages() {
        assert_eq!(
            SomeMatcher {}.failure_message(&None::<&str>),
            String::from("\tExpected:\n\t\tNone\n\tto be a Some")
        );
        assert_eq!(
            SomeMatcher {}.negated_failure_message(&Some("foo")),
            String::from("\tExpected:\n\t\tSome(\"foo\")\n\tnot to be a Some")
        );
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        expect(&Some("thing")).to(be_some())
    }

    #[test]
    fn none_matcher_should_match_if_actual_is_none() {
        assert!(NoneMatcher {}.match_value(&None::<&str>))
    }

    #[test]
    fn none_matcher_should_not_match_if_actual_is_some() {
        assert!(!NoneMatcher {}.match_value(&Some("thing")))
    }

    #[test]
    fn none_matcher_failure_messages() {
        assert_eq!(
            NoneMatcher {}.failure_message(&Some("foo")),
            String::from("\tExpected:\n\t\tSome(\"foo\")\n\tto be None")
        );
        assert_eq!(
            NoneMatcher {}.negated_failure_message(&None::<&str>),
            String::from("\tExpected:\n\t\tNone\n\tnot to be None")
        );
    }

    #[test]
    fn be_none_should_contruct_a_none_matcher() {
        expect(&None::<&str>).to(be_none())
    }
}
