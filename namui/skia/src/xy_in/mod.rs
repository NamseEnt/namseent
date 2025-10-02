mod draw_command;
mod rendering_tree;

use crate::*;
use namui_type::*;
pub use rendering_tree::*;

pub trait XyIn {
    fn xy_in(&self, xy: Xy<Px>) -> bool;
}

impl XyIn for Path {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        NativeCalculate::path_contains_xy(self, None, xy)
    }
}

impl<T> XyIn for &T
where
    T: XyIn,
{
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        T::xy_in(*self, xy)
    }
}
