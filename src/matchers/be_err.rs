use crate::Match;
use crate::Matcher;

pub fn be_err() -> ErrMatcher {
    ErrMatcher {}
}

pub struct ErrMatcher {}

impl<T: std::fmt::Debug, E: std::fmt::Debug> Matcher<Result<T, E>> for ErrMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> Match {
        match actual {
            Err(_) => Match::Matched(format!("expected {:?} not to be an Err", actual)),
            _ => Match::NotMatched(format!("expected {:?} to be an Err", actual)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_err;
    use super::ErrMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_an_err() {
        let actual: Result<u32, String> = Err(String::from("boo"));
        assert_eq!(
            ErrMatcher {}.match_value(&actual),
            Match::Matched(String::from("expected Err(\"boo\") not to be an Err"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_ok() {
        let actual: Result<u32, String> = Ok(42);
        assert_eq!(
            ErrMatcher {}.match_value(&actual),
            Match::NotMatched(String::from("expected Ok(42) to be an Err"))
        )
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        let actual: Result<u32, String> = Err(String::from("boo"));
        expect(&actual).to(be_err())
    }
}
