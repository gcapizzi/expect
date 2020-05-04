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

pub struct Description {
    pub verb: String,
    pub object: Option<String>,
}

/// The contract implemented by matchers.
pub trait Matcher<T> {
    /// Should return `true` if `actual` is a match.
    fn match_value(&self, actual: &T) -> bool;

    /// Should return a [`Description`] of the matcher, to be used for reporting. `actual` _should
    /// not_ be necessary here, but is still passed for type-checking reasons.
    ///
    /// [`Expectation`]: struct.Description.html
    ///
    /// # Example
    ///
    /// ```
    /// # use expect::{Description, Matcher, matchers::EqualMatcher, matchers::equal};
    /// assert_eq!(equal(2).description(&2).verb, String::from("equal"));
    /// assert_eq!(equal(2).description(&2).object, Some(String::from("2")));
    /// ```
    fn description(&self, actual: &T) -> Description;
}

/// Creates an [`Expectation`].
///
/// [`Expectation`]: struct.Expectation.html
pub fn expect<T>(actual: &T) -> Expectation<T> {
    Expectation { actual }
}

/// An expectation. It wraps a value so that it can be checked against a [`Matcher`].
///
/// [`Matcher`]: trait.Matcher.html
pub struct Expectation<'a, T> {
    actual: &'a T,
}

impl<'a, T: std::fmt::Debug> Expectation<'a, T> {
    /// Checks the actual value agains a [`Matcher`], looking for a match.
    ///
    /// If [`Matcher::match_value`] returns `false`, this method will [`panic!`] with a failure
    /// message based on the [`Matcher::description`].
    ///
    /// [`Matcher`]: trait.Matcher.html
    /// [`Matcher::match_value`]: trait.Matcher.html#tymethod.match_value
    /// [`Matcher::description`]: trait.Matcher.html#tymethod.description
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
    pub fn to<M: Matcher<T>>(&self, matcher: M) {
        if !matcher.match_value(&self.actual) {
            fail_test(&self.actual, matcher.description(&self.actual))
        }
    }

    /// Checks the actual value agains a [`Matcher`], looking for the lack of a match.
    ///
    /// If [`Matcher::match_value`] returns `true`, this method will [`panic!`] with a failure
    /// message based on the [`Matcher::description`].
    ///
    /// [`Matcher`]: trait.Matcher.html
    /// [`Matcher::match_value`]: trait.Matcher.html#tymethod.match_value
    /// [`Matcher::description`]: trait.Matcher.html#tymethod.description
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
    pub fn not_to<M: Matcher<T>>(&self, matcher: M) {
        if matcher.match_value(&self.actual) {
            fail_test_negated(&self.actual, matcher.description(&self.actual))
        }
    }
}

fn fail_test<T: std::fmt::Debug>(actual: T, description: Description) {
    panic!(failure_message(actual, description, "to"))
}

fn fail_test_negated<T: std::fmt::Debug>(actual: T, description: Description) {
    panic!(failure_message(actual, description, "not to"))
}

fn failure_message<T: std::fmt::Debug>(
    actual: T,
    description: Description,
    before_verb: &str,
) -> String {
    let predicate = if let Some(obj) = description.object {
        format!("{}:\n\t\t{}", description.verb, obj)
    } else {
        description.verb
    };

    format!(
        "Expectation failed:\n\tExpected:\n\t\t{:?}\n\t{} {}\n",
        actual, before_verb, predicate
    )
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
