use crate::matchers::Collection;
use crate::Matcher;

pub fn contain<T>(element: T) -> ContainMatcher<T> {
    ContainMatcher { element }
}

pub struct ContainMatcher<T> {
    element: T,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug, V: Collection<T> + std::fmt::Debug> Matcher<V>
    for ContainMatcher<T>
{
    fn match_value(&self, collection: &V) -> bool {
        collection.contains_element(&self.element)
    }

    fn failure_message(&self, collection: &V) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tto contain:\n\t\t{:?}",
            collection, self.element
        )
    }

    fn negated_failure_message(&self, collection: &V) -> String {
        format!(
            "\tExpected:\n\t\t{:?}\n\tnot to contain:\n\t\t{:?}",
            collection, self.element
        )
    }
}

#[cfg(test)]
mod tests {
    use super::contain;
    use super::ContainMatcher;
    use crate::expect;
    use crate::Matcher;

    #[test]
    fn should_match_if_collection_contains_element() {
        assert!(ContainMatcher { element: "foo" }.match_value(&vec!["foo"]))
    }

    #[test]
    fn should_not_match_if_collection_does_not_contain_element() {
        assert!(!ContainMatcher { element: "foo" }.match_value(&vec!["bar"]))
    }

    #[test]
    fn failure_messages() {
        assert_eq!(
            ContainMatcher { element: "foo" }.failure_message(&vec!["bar"]),
            String::from("\tExpected:\n\t\t[\"bar\"]\n\tto contain:\n\t\t\"foo\"")
        );
        assert_eq!(
            ContainMatcher { element: "foo" }.negated_failure_message(&vec!["foo"]),
            String::from("\tExpected:\n\t\t[\"foo\"]\n\tnot to contain:\n\t\t\"foo\"")
        );
    }

    #[test]
    fn contain_should_construct_a_contain_matcher() {
        expect(&vec!["foo", "bar"]).to(contain("foo"))
    }
}
