use crate::Matcher;

pub fn be_ok() -> OkMatcher {
    OkMatcher {}
}

pub struct OkMatcher {}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Matcher<Result<T, E>> for OkMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> bool {
        actual.is_ok()
        // match actual {
        //     Ok(_) => Match::Matched(format!("expected {:?} not to be an Ok", actual)),
        //     _ => Match::NotMatched(format!("expected {:?} to be an Ok", actual)),
        // }
    }

    fn failure_message(&self, actual: &Result<T, E>) -> String {
        format!("expected {:?} to be Ok", actual)
    }

    fn negated_failure_message(&self, actual: &Result<T, E>) -> String {
        format!("expected {:?} not to be Ok", actual)
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
        let actual: Result<u32, String> = Ok(42);
        assert!(OkMatcher {}.match_value(&actual))
    }

    #[test]
    fn should_not_match_if_actual_is_err() {
        let actual: Result<u32, String> = Err(String::from("boo"));
        assert!(!OkMatcher {}.match_value(&actual))
    }

    #[test]
    fn failure_messages() {
        let actual: Result<u32, String> = Ok(42);
        assert_eq!(
            OkMatcher {}.failure_message(&actual),
            String::from("expected Ok(42) to be Ok")
        );
        assert_eq!(
            OkMatcher {}.negated_failure_message(&actual),
            String::from("expected Ok(42) not to be Ok")
        );
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        let actual: Result<u32, String> = Ok(42);
        expect(&actual).to(be_ok())
    }
}
