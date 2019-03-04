use crate::Match;
use crate::Matcher;

pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher { expected: expected }
}

pub struct EqualMatcher<T> {
    expected: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Display> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, actual: &T) -> Match {
        if *actual == self.expected {
            Match::Matched(format!(
                "expected {} not to equal {}",
                actual, self.expected
            ))
        } else {
            Match::NotMatched(format!("expected {} to equal {}", actual, self.expected))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::equal;
    use super::EqualMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_equals_expected() {
        assert_eq!(
            EqualMatcher { expected: 42 }.match_value(&42),
            Match::Matched(String::from("expected 42 not to equal 42"))
        )
    }

    #[test]
    fn should_not_match_if_actual_does_not_equal_expected() {
        assert_eq!(
            EqualMatcher { expected: 42 }.match_value(&43),
            Match::NotMatched(String::from("expected 43 to equal 42"))
        )
    }

    #[test]
    fn equal_should_construct_an_equal_matcher() {
        expect(&42).to(equal(42))
    }
}
