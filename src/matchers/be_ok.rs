use crate::Match;
use crate::Matcher;

pub fn be_ok<T>(expected: T) -> OkMatcher<T> {
    OkMatcher { expected: expected }
}

pub struct OkMatcher<T> {
    expected: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug, E: std::fmt::Debug> Matcher<Result<T, E>>
    for OkMatcher<T>
{
    fn match_value(&self, actual: &Result<T, E>) -> Match {
        match actual {
            Ok(actual_value) => if actual_value == &self.expected {
                Match::Matched(format!("expected Ok({:?}) not to be Ok({:?})", actual_value, self.expected))
            } else {
                Match::NotMatched(format!("expected Ok({:?}) to be Ok({:?})", actual_value, self.expected))
            },
            Err(actual_value) => Match::NotMatched(format!("expected Err({:?}) to be Ok({:?})", actual_value, self.expected)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_ok;
    use super::OkMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_ok_and_value_is_equal_to_expected() {
        let actual: Result<u32, String> = Ok(42);
        assert_eq!(
            OkMatcher { expected: 42 }.match_value(&actual),
            Match::Matched(String::from("expected Ok(42) not to be Ok(42)"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_ok_but_value_is_not_equal_to_expected() {
        let actual: Result<u32, String> = Ok(43);
        assert_eq!(
            OkMatcher { expected: 42 }.match_value(&actual),
            Match::NotMatched(String::from("expected Ok(43) to be Ok(42)"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_err() {
        let actual: Result<u32, String> = Err(String::from("boo"));
        assert_eq!(
            OkMatcher { expected: 42 }.match_value(&actual),
            Match::NotMatched(String::from("expected Err(\"boo\") to be Ok(42)"))
        )
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        let actual: Result<u32, String> = Ok(42);
        expect(&actual).to(be_ok(42))
    }
}
