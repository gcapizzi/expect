use crate::Match;
use crate::Matcher;

pub fn be_none() -> NoneMatcher {
    NoneMatcher {}
}

pub struct NoneMatcher {}

impl<T: std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug> Matcher<Option<T>>
    for NoneMatcher
{
    fn match_value(&self, actual: &Option<T>) -> Match {
        if actual.is_none() {
            Match::Matched(format!("expected {:?} not to be None", actual))
        } else {
            Match::NotMatched(format!("expected {:?} to be None", actual))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_none;
    use super::NoneMatcher;
    use crate::expect;
    use crate::Match;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_is_none() {
        let actual: Option<u32> = None;
        assert_eq!(
            NoneMatcher {}.match_value(&actual),
            Match::Matched(String::from("expected None not to be None"))
        )
    }

    #[test]
    fn should_not_match_if_actual_is_some() {
        let actual = Some("thing");
        assert_eq!(
            NoneMatcher {}.match_value(&actual),
            Match::NotMatched(String::from("expected Some(\"thing\") to be None"))
        )
    }

    #[test]
    fn be_none_should_contruct_a_none_matcher() {
        let actual: Option<u32> = None;
        expect(&actual).to(be_none())
    }
}
