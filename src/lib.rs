//! This library provides a way to perform test expectations in a more flexible and powerful way than
//! the built-in `assert*` macros.
//!
//! The basic structure of an expectation is:
//!
//! ```
//! # use expect::{expect, matchers::equal};
//! # let actual = "foo";
//! # let expected = "foo";
//! # let matcher = equal;
//! expect(&actual).to(matcher(expected));
//! # let actual = "foo";
//! # let expected = "bar";
//! expect(&actual).not_to(matcher(expected));
//! ```
//!
//! The library ships with a a number of built-in [matchers], but custom matchers can be
//! implemented by implementing the [`Matcher`] trait.
//!
//! [matchers]: matchers/index.html
//! [`Matcher`]: trait.Matcher.html
pub mod matchers;

/// The contract implemented by matchers.
pub trait Matcher<T> {
    /// Should return `true` if `actual` is a match.
    fn match_value(&self, actual: &T) -> bool;

    /// Should return an appropriate message for a failure to match `actual`, i.e. a failure caused
    /// by `match_value` returning `false`.
    ///
    /// # Example
    ///
    /// ```
    /// # use expect::{Matcher, matchers::equal};
    /// assert_eq!(equal(2).failure_message(&3), String::from("\tExpected:\n\t\t3\n\tto equal:\n\t\t2"));
    /// ```
    fn failure_message(&self, actual: &T) -> String;

    /// Should return an appropriate message for a failure *not* to match `actual`, i.e. a failure
    /// caused by `match_value` returning `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # use expect::{Matcher, matchers::equal};
    /// assert_eq!(equal(2).negated_failure_message(&2), String::from("\tExpected:\n\t\t2\n\tnot to equal:\n\t\t2"));
    /// ```
    fn negated_failure_message(&self, actual: &T) -> String;
}

/// Creates an [`Expectation`].
///
/// [`Expectation`]: struct.Expectation.html
pub fn expect<'a, T>(actual: &'a T) -> Expectation<'a, T> {
    Expectation { actual }
}

/// An expectation. It wraps a value so that it can be checked against a [`Matcher`].
///
/// [`Matcher`]: trait.Matcher.html
pub struct Expectation<'a, T> {
    actual: &'a T,
}

impl<'a, T> Expectation<'a, T> {
    /// Checks the actual value agains a [`Matcher`], looking for a match.
    ///
    /// If [`Matcher::match_value`] returns `false`, this method will [`panic!`] using the failure
    /// message returned by [`Matcher::failure_message`].
    ///
    /// [`Matcher`]: trait.Matcher.html
    /// [`Matcher::match_value`]: trait.Matcher.html#tymethod.match_value
    /// [`Matcher::failure_message`]: trait.Matcher.html#tymethod.failure_message
    /// [`Matcher::negated_failure_message`]: trait.Matcher.html#tymethod.negated_failure_message
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
    pub fn to<M: Matcher<T>>(&self, matcher: M) {
        if !matcher.match_value(&self.actual) {
            fail_test(matcher.failure_message(&self.actual))
        }
    }

    /// Checks the actual value agains a [`Matcher`], looking for the lack of a match.
    ///
    /// If [`Matcher::match_value`] returns `true`, this method will [`panic!`] using the failure
    /// message returned by [`Matcher::negated_failure_message`].
    ///
    /// [`Matcher`]: trait.Matcher.html
    /// [`Matcher::match_value`]: trait.Matcher.html#tymethod.match_value
    /// [`Matcher::failure_message`]: trait.Matcher.html#tymethod.failure_message
    /// [`Matcher::negated_failure_message`]: trait.Matcher.html#tymethod.negated_failure_message
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
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
    use crate::matchers::equal;

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
