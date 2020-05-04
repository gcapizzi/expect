use crate::{Description, Matcher};

/// Matches if `actual` is a [`Result::Ok`].
///
/// [`Restul::Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::result::be_ok};
/// expect(&Ok::<&str, &str>("foo")).to(be_ok());
/// expect(&Err::<&str, &str>("bar")).not_to(be_ok());
/// ```
pub fn be_ok() -> OkMatcher {
    OkMatcher {}
}

pub struct OkMatcher {}

impl<T, E> Matcher<Result<T, E>> for OkMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> bool {
        actual.is_ok()
    }

    fn description(&self, _: &Result<T, E>) -> Description {
        Description {
            verb: String::from("be Ok"),
            object: None,
        }
    }
}

/// Matches if `actual` is a [`Result::Err`].
///
/// [`Restul::Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::result::be_err};
/// expect(&Err::<&str, &str>("foo")).to(be_err());
/// expect(&Ok::<&str, &str>("bar")).not_to(be_err());
/// ```
pub fn be_err() -> ErrMatcher {
    ErrMatcher {}
}

pub struct ErrMatcher {}

impl<T, E> Matcher<Result<T, E>> for ErrMatcher {
    fn match_value(&self, actual: &Result<T, E>) -> bool {
        actual.is_err()
    }

    fn description(&self, _: &Result<T, E>) -> Description {
        Description {
            verb: String::from("be an Err"),
            object: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{be_err, be_ok};
    use crate::Matcher;

    #[test]
    fn ok_matcher_should_match_if_actual_is_ok() {
        assert!(be_ok().match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn ok_matcher_should_not_match_if_actual_is_err() {
        assert!(!be_ok().match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn ok_matcher_should_describe_itself() {
        let description = be_ok().description(&Ok::<u32, &str>(42));
        assert_eq!(description.verb, String::from("be Ok"));
        assert_eq!(description.object, None);
    }

    #[test]
    fn err_matcher_should_match_if_actual_is_an_err() {
        assert!(be_err().match_value(&Err::<u32, &str>("boo")))
    }

    #[test]
    fn err_matcher_should_not_match_if_actual_is_ok() {
        assert!(!be_err().match_value(&Ok::<u32, &str>(42)))
    }

    #[test]
    fn err_matcher_should_describe_itself() {
        let description = be_err().description(&Err::<u32, &str>("foo"));
        assert_eq!(description.verb, String::from("be an Err"));
        assert_eq!(description.object, None);
    }
}
