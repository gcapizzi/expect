pub mod matchers;

pub trait Matcher<T> {
    fn match_value(&self, value: &T) -> Result<(), ()>;
}

pub fn expect<T>(value: T) -> Expectation<T> {
    Expectation{value: value}
}

pub struct Expectation<T> {
    value: T
}

impl <T> Expectation<T> {
    pub fn to<M: Matcher<T>>(&self, matcher: M) {
        assert!(matcher.match_value(&self.value).is_ok())
    }

    pub fn not_to<M: Matcher<T>>(&self, matcher: M) {
        assert!(matcher.match_value(&self.value).is_err())
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;
    use crate::matchers::equal::equal;

    #[test]
    fn expect_to_should_not_panic_if_the_matcher_matches_successfully() {
        expect(2+2).to(equal(4))
    }

    #[test]
    #[should_panic]
    fn expect_to_should_panic_if_the_marcher_fails_to_match() {
        expect(2+2).to(equal(5))
    }

    #[test]
    #[should_panic]
    fn expect_not_to_should_panic_if_the_marcher_matches_successfully() {
        expect(2+2).not_to(equal(4))
    }

    #[test]
    fn expect_not_to_should_not_panic_if_the_matcher_fails_to_match() {
        expect(2+2).not_to(equal(5))
    }
}
