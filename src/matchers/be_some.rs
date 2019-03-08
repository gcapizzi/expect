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
        format!("expected {:?} to be a Some", actual)
    }

    fn negated_failure_message(&self, actual: &Option<T>) -> String {
        format!("expected {:?} not to be a Some", actual)
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
        let actual = Some("foo");
        assert!(SomeMatcher {}.match_value(&actual))
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        let actual: Option<String> = None;
        assert!(!SomeMatcher {}.match_value(&actual))
    }

    #[test]
    fn failure_messages() {
        let actual = Some("foo");
        assert_eq!(
            SomeMatcher {}.failure_message(&actual),
            String::from("expected Some(\"foo\") to be a Some")
        );
        assert_eq!(
            SomeMatcher {}.negated_failure_message(&actual),
            String::from("expected Some(\"foo\") not to be a Some")
        );
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        expect(&Some("thing")).to(be_some())
    }
}
