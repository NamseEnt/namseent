#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct Ltrb<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl<T: Copy> Copy for Ltrb<T> {}

impl<T: Default> Default for Ltrb<T> {
    fn default() -> Self {
        Self {
            left: Default::default(),
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default(),
        }
    }
}
