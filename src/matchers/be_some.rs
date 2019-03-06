use crate::Match;
use crate::Matcher;

pub fn be_some<T>(expected: T) -> SomeMatcher<T> {
    SomeMatcher { expected: expected }
}

pub struct SomeMatcher<T> {
    expected: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> Matcher<Option<T>> for SomeMatcher<T> {
    fn match_value(&self, actual: &Option<T>) -> Match {
        match actual {
            Some(actual_value) => {
                if actual_value == &self.expected {
                    Match::Matched(format!(
                        "expected Some({:?}) not to be Some({:?})",
                        actual_value, self.expected
                    ))
                } else {
                    Match::NotMatched(format!(
                        "expected Some({:?}) to be Some({:?})",
                        actual_value, self.expected
                    ))
                }
            }
            None => Match::NotMatched(format!("expected None to be Some({:?})", self.expected)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_some;
    use super::SomeMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_some_and_value_is_equal_to_expected() {
        let actual = Some(42);
        assert_eq!(
            SomeMatcher { expected: 42 }.match_value(&actual),
            Match::Matched(String::from("expected Some(42) not to be Some(42)"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_some_but_value_is_not_equal_to_expected() {
        let actual = Some(43);
        assert_eq!(
            SomeMatcher { expected: 42 }.match_value(&actual),
            Match::NotMatched(String::from("expected Some(43) to be Some(42)"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        let actual: Option<u32> = None;
        assert_eq!(
            SomeMatcher { expected: 42 }.match_value(&actual),
            Match::NotMatched(String::from("expected None to be Some(42)"))
        )
    }

    #[test]
    fn be_some_should_contruct_a_some_matcher() {
        expect(&Some("thing")).to(be_some("thing"))
    }
}
