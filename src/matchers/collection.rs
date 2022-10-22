use crate::{Description, Matcher};

/// Matches if `actual` contains `element`.
///
/// Supports [arrays] [`Vec`]s, [`VecDeque`]s, [`LinkedList`]s, [`HashSet`]s and [`BTreeSet`]s.
///
/// [array]: https://doc.rust-lang.org/std/primitive.array.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
///
/// Examples
///
/// ```
/// # use expect::{expect, matchers::collection::contain};
/// expect(&[1, 2, 3]).to(contain(2));
/// expect(&vec![1, 2, 3]).not_to(contain(4));
/// ```
pub fn contain<T>(element: T) -> ContainMatcher<T> {
    ContainMatcher { element }
}

pub struct ContainMatcher<T> {
    element: T,
}

impl<T: std::fmt::Debug, V: Container<T>> Matcher<V> for ContainMatcher<T> {
    fn match_value(&self, collection: &V) -> bool {
        collection.contains_element(&self.element)
    }

    fn description(&self, _: &V) -> Description {
        Description {
            verb: String::from("contain"),
            object: Some(format!("{:?}", self.element)),
        }
    }
}

pub trait Container<T> {
    fn contains_element(&self, element: &T) -> bool;
}

impl<T: std::cmp::PartialEq, const N: usize> Container<T> for [T; N] {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T: std::cmp::PartialEq> Container<T> for std::vec::Vec<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T: std::cmp::PartialEq> Container<T> for std::collections::VecDeque<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T: std::cmp::PartialEq> Container<T> for std::collections::LinkedList<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T: std::cmp::Eq + std::hash::Hash> Container<T> for std::collections::HashSet<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T: std::cmp::Ord> Container<T> for std::collections::BTreeSet<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

/// Matches if `actual` is empty.
///
/// Supports [arrays], [`Vec`]s, [`VecDeque`]s, [`LinkedList`]s, [`HashSet`]s and [`BTreeSet`]s.
///
/// [array]: https://doc.rust-lang.org/std/primitive.array.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
/// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
/// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
///
/// Examples
///
/// ```
/// # use expect::{expect, matchers::collection::be_empty};
/// expect(&Vec::<i32>::new()).to(be_empty());
/// ```
pub fn be_empty() -> BeEmptyMatcher {
    BeEmptyMatcher {}
}

pub struct BeEmptyMatcher {}

impl<V: Emptiable> Matcher<V> for BeEmptyMatcher {
    fn match_value(&self, collection: &V) -> bool {
        collection.empty()
    }

    fn description(&self, _: &V) -> Description {
        Description {
            verb: String::from("be empty"),
            object: None,
        }
    }
}

pub trait Emptiable {
    fn empty(&self) -> bool;
}

impl<T, const N: usize> Emptiable for [T; N] {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for std::vec::Vec<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for std::collections::VecDeque<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for std::collections::LinkedList<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for std::collections::HashSet<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for std::collections::BTreeSet<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{be_empty, contain, Container, Emptiable};
    use crate::Matcher;

    #[test]
    fn contain_matcher_should_match_if_collection_contains_element() {
        assert!(contain("foo").match_value(&vec!["foo"]))
    }

    #[test]
    fn contain_matcher_should_not_match_if_collection_does_not_contain_element() {
        assert!(!contain("foo").match_value(&vec!["bar"]))
    }

    #[test]
    fn contain_matcher_should_describe_itself() {
        let description = contain("foo").description(&vec!["bar"]);
        assert_eq!(description.verb, String::from("contain"));
        assert_eq!(description.object, Some(String::from("\"foo\"")));
    }

    #[test]
    fn be_empty_matcher_should_match_if_collection_is_empty() {
        assert!(be_empty().match_value(&std::vec::Vec::<i32>::new()))
    }

    #[test]
    fn be_empty_matcher_should_not_match_if_collection_is_not_empty() {
        assert!(!be_empty().match_value(&vec![42]))
    }

    #[test]
    fn be_empty_matcher_should_describe_itself() {
        let description = be_empty().description(&vec!["foo"]);
        assert_eq!(description.verb, String::from("be empty"));
        assert_eq!(description.object, None);
    }

    #[test]
    fn arrays_are_containers() {
        assert!([1, 2, 3].contains_element(&2));
        assert!(![1, 2, 3].contains_element(&4));
    }

    #[test]
    fn arrays_are_emptiable() {
        assert!(([] as [u64; 0]).empty());
        assert!(![1, 2, 3].empty());
    }

    #[test]
    fn vecs_are_containers() {
        assert!(vec![1, 2, 3].contains_element(&2));
        assert!(!vec![1, 2, 3].contains_element(&4));
    }

    #[test]
    fn vecs_are_emptiable() {
        assert!(std::vec::Vec::<i32>::new().empty());
        assert!(!vec![1, 2, 3].empty());
    }

    #[test]
    fn vecdeques_are_containers() {
        let mut numbers = std::collections::VecDeque::new();
        numbers.push_back(1);
        numbers.push_back(2);
        numbers.push_back(3);

        assert!(numbers.contains_element(&2));
        assert!(!numbers.contains_element(&4));
    }

    #[test]
    fn vecdeques_are_emptiable() {
        let mut numbers = std::collections::VecDeque::new();
        assert!(numbers.empty());

        numbers.push_back(42);
        assert!(!numbers.empty());
    }

    #[test]
    fn linkedlists_are_containers() {
        let mut numbers = std::collections::LinkedList::new();
        numbers.push_back(1);
        numbers.push_back(2);
        numbers.push_back(3);

        assert!(numbers.contains_element(&2));
        assert!(!numbers.contains_element(&4));
    }

    #[test]
    fn linkedlists_are_emptiable() {
        let mut numbers = std::collections::LinkedList::new();
        assert!(numbers.empty());

        numbers.push_back(42);
        assert!(!numbers.empty())
    }

    #[test]
    fn hashsets_are_containers() {
        let mut numbers = std::collections::HashSet::new();
        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);

        assert!(numbers.contains_element(&2));
        assert!(!numbers.contains_element(&4));
    }

    #[test]
    fn hashsets_are_emptiable() {
        let mut numbers = std::collections::HashSet::new();
        assert!(numbers.empty());

        numbers.insert(42);
        assert!(!numbers.empty())
    }

    #[test]
    fn btreesets_are_containers() {
        let mut numbers = std::collections::BTreeSet::new();
        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);

        assert!(numbers.contains_element(&2));
        assert!(!numbers.contains_element(&4));
    }

    #[test]
    fn btreesets_are_emptiable() {
        let mut numbers = std::collections::BTreeSet::new();
        assert!(numbers.empty());

        numbers.insert(42);
        assert!(!numbers.empty())
    }
}
