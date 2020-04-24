use crate::Matcher;

use regex::Regex;

/// Matches if the provided regular expression matches the actual value.
///
/// `regex` needs to be a valid regular expression and the matching is performed using the
/// [`is_match`] method.
///
/// [`is_match`]: https://docs.rs/regex/1.3.7/regex/struct.Regex.html#method.is_match
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::string::match_regex};
/// expect(&"abc-123").to(match_regex(r"^[a-z]{3}-\d{3}$"));
/// expect(&"abc-123").to(match_regex(r"\d{3}"));
/// ```
pub fn match_regex<S: AsRef<str>>(regex: S) -> MatchRegexMatcher<S> {
    MatchRegexMatcher { regex }
}

pub struct MatchRegexMatcher<S> {
    regex: S,
}

impl<A: AsRef<str> + std::fmt::Debug, E: AsRef<str> + std::fmt::Debug> Matcher<A>
    for MatchRegexMatcher<E>
{
    fn match_value(&self, actual: &A) -> bool {
        if let Ok(compiled_regex) = self.regex.as_ref().parse::<Regex>() {
            return compiled_regex.is_match(actual.as_ref());
        }
        return false;
    }

    fn failure_message(&self, actual: &A) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tto match regex:\n\t\t{:?}",
            actual, self.regex,
        )
    }

    fn negated_failure_message(&self, actual: &A) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tnot to match regex:\n\t\t{:?}",
            actual, self.regex,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::match_regex;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_matches_with_regex() {
        assert!(match_regex("foo.*").match_value(&"foobar"))
    }

    #[test]
    fn should_not_match_if_actual_does_not_match_regex() {
        assert!(!match_regex("foo.*").match_value(&"bar"))
    }

    #[test]
    fn failure_messages() {
        assert_eq!(
            match_regex("foo.*").failure_message(&"bar"),
            String::from("\tExpected:\n\t\t\"bar\"\n\tto match regex:\n\t\t\"foo.*\"")
        );
        assert_eq!(
            match_regex("foo.*").negated_failure_message(&"foobar"),
            String::from("\tExpected:\n\t\t\"foobar\"\n\tnot to match regex:\n\t\t\"foo.*\"")
        );
    }
}
