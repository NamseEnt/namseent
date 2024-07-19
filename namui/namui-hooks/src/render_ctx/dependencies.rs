use crate::*;

pub trait Dependencies<T> {
    fn cloned(self) -> T;
}

impl<T, U> Dependencies<U> for &T
where
    T: ToOwned<Owned = U> + ?Sized,
    U: 'static,
{
    fn cloned(self) -> U {
        self.to_owned()
    }
}

impl<T, R> Dependencies<T> for Sig<'_, T, R>
where
    T: 'static + Send + Clone,
    R: std::borrow::Borrow<T>,
{
    fn cloned(self) -> T {
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
                $($T: Dependencies<$R>,)*
                $($R: 'static,)*
            > Dependencies<($($R,)*)> for ($($T,)*)
            {
                fn cloned(self) -> ($($R,)*) {
                    ($(self.$i.cloned(),)*)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_dependencies_should_be_string() {
        let str_ref = "hello";
        let cloned: String = str_ref.cloned();
    }

    #[test]
    fn sig_dependencies_should_be_inner() {
        let world = World::new();
        let sig = Sig::new(1, SigId::new(), &world);
        let cloned: i32 = sig.cloned();
    }
}
