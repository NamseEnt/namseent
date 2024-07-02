use crate::*;

pub trait Dependencies<T: 'static> {
    fn cloned(&self) -> T;
}
impl Dependencies<()> for () {
    fn cloned(&self) {}
}
impl<T: 'static + Send + Clone, R: std::borrow::Borrow<T>> Dependencies<T> for Sig<'_, T, R> {
    fn cloned(&self) -> T {
        self.as_ref().clone()
    }
}

macro_rules! dependencies_impl {
    (
        $(
            ($
                ($T:ident, $i:tt, $R: ident),
            *),
        )*
    ) => {
        $(
            impl<
                $($T: 'static + Send + Clone,)*
                $($R: std::borrow::Borrow<$T>,)*
            > Dependencies<($($T,)*)> for ($(Sig<'_, $T, $R>,)*)
            {
                fn cloned(&self) -> ($($T,)*) {
                    ($(self.$i.as_ref().clone(),)*)
                }
            }
        )*
    };
}

dependencies_impl!(
    (T0, 0, R0),
    (T0, 0, R0, T1, 1, R1),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5, T6, 6, R6),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5, T6, 6, R6, T7, 7, R7),
);
