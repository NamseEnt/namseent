use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CheapSlice<T> {
    source: Arc<Box<[T]>>,
    offset: usize,
    len: usize,
}

impl<T> CheapSlice<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn slice(&self, offset: usize, len: usize) -> Self {
        let offset = (self.offset + offset).min(self.source.len());
        let len = len.min(self.source.len() - offset);

        Self {
            source: self.source.clone(),
            offset,
            len,
        }
    }

    pub fn split(&self, at: usize) -> (Self, Self) {
        let left = self.slice(0, at);
        let right = self.slice(at, self.len.saturating_sub(at));
        (left, right)
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            len: vec.len(),
            source: Arc::new(vec.into_boxed_slice()),
            offset: 0,
        }
    }

    pub fn slice_front(&mut self, count: usize) -> Self {
        let (fronts, rest) = self.split(count);
        *self = rest;
        fronts
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.source[self.offset..self.offset + self.len].to_vec()
    }
}

impl<T> std::ops::Deref for CheapSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.source[self.offset..self.offset + self.len]
    }
}

impl<T: Copy> IntoIterator for CheapSlice<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[allow(clippy::unnecessary_to_owned)] // NOTE: I don't know how to do without this.
    fn into_iter(self) -> Self::IntoIter {
        self.source[self.offset..self.offset + self.len]
            .to_vec()
            .into_iter()
    }
}

impl<T> From<Vec<T>> for CheapSlice<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_vec(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_does_not_affect_source() {
        let source = CheapSlice::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let sliced = source.slice(2, 5);
        assert_eq!(source.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(sliced.to_vec(), vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn slice_front_does_not_affect_source() {
        let mut source = CheapSlice::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let sliced = source.slice_front(5);
        assert_eq!(source.to_vec(), vec![6, 7, 8, 9]);
        assert_eq!(sliced.to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn split_does_not_affect_source() {
        let source = CheapSlice::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let (left, right) = source.split(5);
        assert_eq!(source.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(left.to_vec(), vec![1, 2, 3, 4, 5]);
        assert_eq!(right.to_vec(), vec![6, 7, 8, 9]);
    }

    #[test]
    fn slice_len_overflows_no_panic() {
        let source = CheapSlice::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let sliced = source.slice(2, 100);
        assert_eq!(source.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(sliced.to_vec(), vec![3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn slice_offset_overflows_no_panic() {
        let source = CheapSlice::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let sliced = source.slice(100, 5);
        assert_eq!(source.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(sliced.to_vec(), Vec::<i32>::new());
    }
}
