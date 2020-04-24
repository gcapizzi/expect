use crate::Matcher;

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

impl<T: std::fmt::Debug + AsRef<Path>> Matcher<T> for ExistMatcher {
    fn match_value(&self, actual: &T) -> bool {
        actual.as_ref().exists()
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tto exist", actual)
    }

    fn negated_failure_message(&self, actual: &T) -> String {
        format!("\tExpected:\n\t\t{:?}\n\tnot to exist", actual)
    }
}

#[cfg(test)]
mod tests {
    use super::exist;
    use crate::Matcher;

    #[test]
    fn should_match_if_actual_exists() {
        assert!(exist().match_value(&"./Cargo.toml"))
    }

    #[test]
    fn should_not_match_if_actual_is_none() {
        assert!(!exist().match_value(&"does_not_exist"))
    }

    #[test]
    fn failure_messages() {
        assert_eq!(
            exist().failure_message(&"does_not_exist"),
            String::from("\tExpected:\n\t\t\"does_not_exist\"\n\tto exist")
        );
        assert_eq!(
            exist().negated_failure_message(&"does_exist"),
            String::from("\tExpected:\n\t\t\"does_exist\"\n\tnot to exist")
        );
    }
}
