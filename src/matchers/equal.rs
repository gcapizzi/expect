use crate::Matcher;

pub fn equal<T>(value: T) -> EqualMatcher<T> {
    EqualMatcher{value: value}
}

pub struct EqualMatcher<T> {
    value: T
}

impl <T: std::cmp::PartialEq> Matcher<T> for EqualMatcher<T> {
    fn match_value(&self, value: &T) -> Result<(), ()> {
        if *value == self.value {
            Ok(())
        } else {
            Err(())
        }
    }
}

