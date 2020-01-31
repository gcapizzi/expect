pub mod matchers;

pub trait Matcher<T> {
    fn match_value(&self, actual: &T) -> bool;
    fn failure_message(&self, actual: &T) -> String;
    fn negated_failure_message(&self, actual: &T) -> String;
}

pub fn expect<'a, T>(actual: &'a T) -> Expectation<'a, T> {
    Expectation { actual }
}

pub struct Expectation<'a, T> {
    actual: &'a T,
}

impl<'a, T> Expectation<'a, T> {
    pub fn to<M: Matcher<T>>(&self, matcher: M) {
        if !matcher.match_value(&self.actual) {
            fail_test(matcher.failure_message(&self.actual))
        }
    }

    pub fn not_to<M: Matcher<T>>(&self, matcher: M) {
        if matcher.match_value(&self.actual) {
            fail_test(matcher.negated_failure_message(&self.actual))
        }
    }
}

fn fail_test(message: String) {
    panic!("Expectation failed:\n{}\n", message)
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
    #[should_panic(expected = "Expectation failed")]
    fn expect_to_should_panic_if_the_matcher_fails_to_match() {
        expect(&(2 + 2)).to(equal(5))
    }

    #[test]
    #[should_panic(expected = "Expectation failed")]
    fn expect_not_to_should_panic_if_the_matcher_matches_successfully() {
        expect(&(2 + 2)).not_to(equal(4))
    }

    #[test]
    fn expect_not_to_should_not_panic_if_the_matcher_fails_to_match() {
        expect(&(2 + 2)).not_to(equal(5))
    }
}
