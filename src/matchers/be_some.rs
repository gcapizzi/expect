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

#[cfg(test)]
mod tests {
    use super::be_some;
    use super::SomeMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_some() {
        assert!(SomeMatcher {}.match_value(&Some("foo")))
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        assert!(!SomeMatcher {}.match_value(&None::<u32>))
    }

    #[test]
    fn failure_messages() {
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
}
