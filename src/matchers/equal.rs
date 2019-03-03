use crate::Matcher;

pub fn equal<T>(value: T) -> EqualMatcher<T> {
    EqualMatcher { value: value }
}

pub struct EqualMatcher<T> {
    value: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Display> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, value: &T) -> Result<String, String> {
        if *value == self.value {
            Ok(format!("expected {} not to equal {}", value, self.value))
        } else {
            Err(format!("expected {} to equal {}", value, self.value))
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
            EqualMatcher { value: 42 }.match_value(&42),
            Ok(String::from("expected 42 not to equal 42"))
        )
    }

    #[test]
    fn match_value_should_fail_if_actual_does_not_equal_expected() {
        assert_eq!(
            EqualMatcher { value: 42 }.match_value(&43),
            Err(String::from("expected 43 to equal 42"))
        )
    }
}
