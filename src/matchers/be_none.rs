use crate::Matcher;

pub fn be_none() -> NoneMatcher {
    NoneMatcher {}
}

pub struct NoneMatcher {}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Matcher<Option<T>> for NoneMatcher {
    fn match_value(&self, actual: &Option<T>) -> bool {
        actual.is_none()
    }

    fn failure_message(&self, actual: &Option<T>) -> String {
        format!("expected {:?} to be None", actual)
    }

    fn negated_failure_message(&self, actual: &Option<T>) -> String {
        format!("expected {:?} not to be None", actual)
    }
}

#[cfg(test)]
mod tests {
    use super::be_none;
    use super::NoneMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_none() {
        let actual: Option<u32> = None;
        assert!(NoneMatcher {}.match_value(&actual))
    }

    #[test]
    fn should_not_match_if_actual_is_some() {
        let actual = Some("thing");
        assert!(!NoneMatcher {}.match_value(&actual))
    }

    #[test]
    fn failure_messages() {
        let actual: Option<u32> = None;
        assert_eq!(
            NoneMatcher {}.failure_message(&actual),
            String::from("expected None to be None")
        );
        assert_eq!(
            NoneMatcher {}.negated_failure_message(&actual),
            String::from("expected None not to be None")
        );
    }

    #[test]
    fn be_none_should_contruct_a_none_matcher() {
        let actual: Option<u32> = None;
        expect(&actual).to(be_none())
    }
}
