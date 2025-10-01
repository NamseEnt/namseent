mod draw_command;
mod rendering_tree;

use crate::*;
pub use rendering_tree::*;

pub trait XyIn {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool;
}

impl XyIn for Path {
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        calculator.path_contains_xy(self, None, xy)
    }
}

impl<T> XyIn for &T
where
    T: XyIn,
{
    fn xy_in(&self, calculator: &dyn SkCalculate, xy: Xy<Px>) -> bool {
        T::xy_in(*self, calculator, xy)
    }
}
