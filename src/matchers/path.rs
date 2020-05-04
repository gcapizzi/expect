use crate::{Description, Matcher};

use std::path::Path;

/// Matches if the actual path points to en existing file or directory
///
/// # Examples
///
/// ```
/// # use expect::{expect, matchers::path::exist};
/// expect(&env!("CARGO_HOME")).to(exist());
/// expect(&"/does/not/exist").not_to(exist());
/// ```
pub fn exist() -> ExistMatcher {
    ExistMatcher {}
}

pub struct ExistMatcher {}

impl<T: AsRef<Path>> Matcher<T> for ExistMatcher {
    fn match_value(&self, actual: &T) -> bool {
        actual.as_ref().exists()
    }

    fn description(&self, _: &T) -> Description {
        Description {
            verb: String::from("exist"),
            object: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::exist;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_exists() {
        assert!(exist().match_value(&env!("CARGO_HOME")))
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        assert!(!exist().match_value(&"does_not_exist"))
    }

    #[test]
    fn should_describe_itself() {
        let description = exist().description(&"bar");
        assert_eq!(description.verb, String::from("exist"));
        assert_eq!(description.object, None);
    }
}
