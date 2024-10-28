//! # Set
//!
//! Set is a collection of unique elements.
//! Its internal implement is the List.

use crate::*;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Set<T: Nsd + Eq> {
    list: List<T>,
}

impl<T: Nsd + Eq> Set<T> {
    pub fn new() -> Self {
        Self { list: List::new() }
    }

    pub fn insert(&mut self, value: T) {
        if self.list.contains(&value) {
            return;
        }
        self.list.push(value);
    }

    pub fn contains<Q>(&self, value: Q) -> bool
    where
        Q: Borrow<T>,
    {
        self.list.contains(value.borrow())
    }

    pub fn remove(&mut self, value: &T) {
        self.list.retain(|v| v != value);
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.list.iter()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl<T: Nsd + Eq> Default for Set<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Nsd + Eq> Nsd for Set<T> {
    fn byte_len(&self) -> usize {
        self.list.byte_len()
    }

    fn write_on_bytes(&self, bytes: &mut [u8]) -> usize {
        self.list.write_on_bytes(bytes)
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, FromBytesError>
    where
        Self: Sized,
    {
        Ok(Self {
            list: List::from_bytes(bytes)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let bytes = {
            let mut doc = Set::<usize>::new();

            assert!(doc.is_empty());

            doc.insert(0);

            assert!(doc.contains(0));
            assert!(!doc.contains(1));
            assert!(doc.len() == 1);

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 3);

        let bytes = {
            let doc = Set::<usize>::from_bytes(bytes);

            assert!(doc.contains(0));
            assert!(!doc.contains(1));
            assert!(doc.len() == 1);

            let mut doc = doc;

            doc.remove(&0);

            assert_eq!(doc.len(), 0);
            assert!(!doc.contains(0));
            assert!(!doc.contains(1));

            doc.insert(3);
            assert_eq!(doc.len(), 1);
            assert!(!doc.contains(0));
            assert!(!doc.contains(1));
            assert!(!doc.contains(2));
            assert!(doc.contains(3));
            assert!(!doc.contains(4));

            doc.insert(4);

            assert_eq!(doc.len(), 2);
            assert!(!doc.contains(0));
            assert!(!doc.contains(1));
            assert!(!doc.contains(2));
            assert!(doc.contains(3));
            assert!(doc.contains(4));

            doc.to_bytes()
        };

        assert_eq!(bytes.len(), 13);
    }
}
