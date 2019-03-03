use crate::Matcher;

pub fn equal<T>(value: T) -> EqualMatcher<T> {
    EqualMatcher{value: value}
}

pub struct EqualMatcher<T> {
    value: T
}

impl <T: std::cmp::PartialEq + std::fmt::Display> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, value: &T) -> Result<String, String> {
        if *value == self.value {
            Ok(format!("expected {} not to equal {}", value, self.value))
        } else {
            Err(format!("expected {} to equal {}", value, self.value))
        }
    }
}

