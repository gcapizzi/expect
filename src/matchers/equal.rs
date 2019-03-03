use crate::Matcher;

pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher { expected: expected }
}

pub struct EqualMatcher<T> {
    expected: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Display> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, actual: &T) -> Result<String, String> {
        if *actual == self.expected {
            Ok(format!("expected {} not to equal {}", actual, self.expected))
        } else {
            Err(format!("expected {} to equal {}", actual, self.expected))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EqualMatcher;
    use crate::Matcher;

    #[test]
    fn match_value_should_succeed_if_actual_equals_expected() {
        assert_eq!(
            EqualMatcher { expected: 42 }.match_value(&42),
            Ok(String::from("expected 42 not to equal 42"))
        )
    }

    #[test]
    fn match_value_should_fail_if_actual_does_not_equal_expected() {
        assert_eq!(
            EqualMatcher { expected: 42 }.match_value(&43),
            Err(String::from("expected 43 to equal 42"))
        )
    }
}
