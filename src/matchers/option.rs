use crate::{Description, Matcher};

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

#[cfg(test)]
mod tests {
    use super::{be_none, be_some};
    use crate::Matcher;

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
}
