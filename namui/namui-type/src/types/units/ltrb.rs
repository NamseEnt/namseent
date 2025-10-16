use crate::*;
use std::fmt::Debug;

#[type_derives(Copy)]
pub struct Ltrb<T>
where
    T: Debug + State,
{
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl<T> Ltrb<T>
where
    T: Clone + Debug + State,
{
    pub fn all(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            left: value.clone(),
            top: value.clone(),
            right: value.clone(),
            bottom: value,
        }
    }
}

impl<T: Default> Default for Ltrb<T>
where
    T: Default + Debug + State,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default(),
        }
    }
}
