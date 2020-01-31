pub mod be_err;
pub mod be_none;
pub mod be_ok;
pub mod be_some;
pub mod equal;
pub mod contain;

pub trait Collection<T> {
    fn contains_element(&self, element: &T) -> bool;
}

impl<T: std::cmp::PartialEq> Collection<T> for std::vec::Vec<T> {
    fn contains_element(&self, element: &T) -> bool {
        self.contains(element)
    }
}

#[cfg(test)]
mod tests {
    use super::Collection;

    #[test]
    fn vecs_are_collections() {
        assert!(vec![1, 2, 3].contains_element(&2))
    }
}

