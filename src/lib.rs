pub mod matchers;

#[derive(PartialEq, Debug)]
pub enum Match {
    Matched(String),
    NotMatched(String),
}

pub trait Matcher<T> {
    fn match_value(&self, actual: &T) -> Match;
}

pub fn expect<'a, T>(actual: &'a T) -> Expectation<'a, T> {
    Expectation { actual: actual }
}

pub struct Expectation<'a, T: 'a> {
    actual: &'a T,
}

impl<'a, T> Expectation<'a, T> {
    pub fn to<M: Matcher<T>>(&self, matcher: M) {
        if let Match::NotMatched(positive_failure_message) = matcher.match_value(&self.actual) {
            panic!(positive_failure_message)
        }
    }

    pub fn not_to<M: Matcher<T>>(&self, matcher: M) {
        if let Match::Matched(negative_failure_message) = matcher.match_value(&self.actual) {
            panic!(negative_failure_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;
    use crate::matchers::equal::equal;

    #[test]
    fn expect_to_should_not_panic_if_the_matcher_matches_successfully() {
        expect(&(2 + 2)).to(equal(4))
    }

    #[test]
    #[should_panic(expected = "expected 4 to equal 5")]
    fn expect_to_should_panic_if_the_marcher_fails_to_match() {
        expect(&(2 + 2)).to(equal(5))
    }

    #[test]
    #[should_panic(expected = "expected 4 not to equal 4")]
    fn expect_not_to_should_panic_if_the_marcher_matches_successfully() {
        expect(&(2 + 2)).not_to(equal(4))
    }

    #[test]
    fn expect_not_to_should_not_panic_if_the_matcher_fails_to_match() {
        expect(&(2 + 2)).not_to(equal(5))
    }

    #[test]
    fn it_should_not_take_ownership_of_the_actual_value() {
        let s = String::from("hello");
        expect(&s).to(equal(String::from("hello")));
        expect(&s).not_to(equal(String::from("bye")));
    }
}
