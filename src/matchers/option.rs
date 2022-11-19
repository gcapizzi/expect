use crate::{
    matchers::{equal, EqualMatcher},
    Description, Matcher,
};

/// Matches if `actual` is an [`Option::Some`].
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::option::be_some};
/// expect(&Some("foo")).to(be_some());
/// expect(&None::<&str>).not_to(be_some());
/// ```
pub fn be_some() -> SomeMatcher {
    SomeMatcher {}
}

pub struct SomeMatcher {}

impl<T> Matcher<Option<T>> for SomeMatcher {
    fn match_value(&self, actual: &Option<T>) -> bool {
        actual.is_some()
    }

    fn description(&self, _: &Option<T>) -> Description {
        Description {
            verb: String::from("be a Some"),
            object: None,
        }
    }
}

/// Matches if `actual` is an [`Option::Some`] *and* the contained value matches the inner matcher.
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::{equal, collection::contain, option::match_some,
/// string::match_regex}};
/// expect(&Some(vec![1, 2, 3])).to(match_some(contain(2)));
/// expect(&Some("foo")).not_to(match_some(match_regex("\\d+")));
/// expect(&None::<&str>).not_to(match_some(equal("foo")));
/// ```
pub fn match_some<I>(inner: I) -> MatchSomeMatcher<I> {
    MatchSomeMatcher { inner }
}

pub struct MatchSomeMatcher<I> {
    inner: I,
}

impl<T: std::fmt::Debug, M: Matcher<T>> Matcher<Option<T>> for MatchSomeMatcher<M> {
    fn match_value(&self, actual: &Option<T>) -> bool {
        if let Some(value) = actual {
            return self.inner.match_value(value);
        }
        false
    }

    fn description(&self, actual: &Option<T>) -> Description {
        if let Some(value) = actual {
            let inner_desc = self.inner.description(value);
            Description {
                verb: format!("be a Some and {}", inner_desc.verb),
                object: inner_desc.object,
            }
        } else {
            Description {
                verb: String::from("be a Some"),
                object: None,
            }
        }
    }
}

/// Matches if `actual` is an [`Option::None`].
///
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::option::be_none};
/// expect(&None::<&str>).to(be_none());
/// expect(&Some("foo")).not_to(be_none());
/// ```
pub fn be_none() -> NoneMatcher {
    NoneMatcher {}
}

pub struct NoneMatcher {}

impl<T> Matcher<Option<T>> for NoneMatcher {
    fn match_value(&self, actual: &Option<T>) -> bool {
        actual.is_none()
    }

    fn description(&self, _: &Option<T>) -> Description {
        Description {
            verb: String::from("be None"),
            object: None,
        }
    }
}

/// Matches if `actual` is an [`Option::Some`] *and* the contained value is equal to the actual
/// value. `equal_some(x)` is effectively equivalent to `match_some(equal(x))`.
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::option::equal_some};
/// expect(&Some("foo".to_string())).to(equal_some("foo"));
/// expect(&None::<&str>).not_to(equal_some("foo"));
/// ```
pub fn equal_some<I>(inner: I) -> MatchSomeMatcher<EqualMatcher<I>> {
    match_some(equal(inner))
}

#[cfg(test)]
mod tests {
    use super::{be_none, be_some, match_some};
    use crate::{matchers::equal, Matcher};

    #[test]
    fn some_matcher_should_match_if_actual_is_some() {
        assert!(be_some().match_value(&Some("foo")))
    }

    #[test]
    fn some_matcher_should_not_match_if_actual_is_none() {
        assert!(!be_some().match_value(&None::<u32>))
    }

    #[test]
    fn some_matcher_should_describe_itself() {
        let description = be_some().description(&Some("bar"));
        assert_eq!(description.verb, String::from("be a Some"));
        assert_eq!(description.object, None);
    }

    #[test]
    fn none_matcher_should_match_if_actual_is_none() {
        assert!(be_none().match_value(&None::<&str>))
    }

    #[test]
    fn none_matcher_should_not_match_if_actual_is_some() {
        assert!(!be_none().match_value(&Some("thing")))
    }

    #[test]
    fn none_matcher_should_describe_itself() {
        let description = be_none().description(&None::<&str>);
        assert_eq!(description.verb, String::from("be None"));
        assert_eq!(description.object, None);
    }

    #[test]
    fn match_some_matcher_should_match_if_actual_is_some_and_inner_value_matches_inner_matcher() {
        assert!(match_some(equal("foo")).match_value(&Some("foo")))
    }

    #[test]
    fn match_some_matcher_should_not_match_if_actual_is_some_but_inner_value_does_not_match_inner_matcher(
    ) {
        assert!(!match_some(equal("foo")).match_value(&Some("bar")))
    }

    #[test]
    fn match_some_matcher_should_not_match_if_actual_is_not_some() {
        assert!(!match_some(equal("foo")).match_value(&None::<&str>))
    }

    #[test]
    fn match_some_matcher_should_describe_itself_when_actual_is_not_some() {
        let description = match_some(equal("foo")).description(&None::<&str>);
        assert_eq!(description.verb, String::from("be a Some"));
        assert_eq!(description.object, None)
    }

    #[test]
    fn match_some_matcher_should_describe_itself_and_its_inner_matcher_when_actual_is_some() {
        let description = match_some(equal("foo")).description(&Some("foo"));
        assert_eq!(description.verb, String::from("be a Some and equal"));
        assert_eq!(description.object, Some(String::from("\"foo\"")))
    }
}
