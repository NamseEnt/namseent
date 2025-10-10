use crate::*;

pub trait Dependencies {
    type Owned;
    fn to_owned(self) -> Self::Owned;
}

impl<T> Dependencies for Sig<'_, T>
where
    T: Clone,
{
    type Owned = T;
    fn to_owned(self) -> Self::Owned {
        self.clone_inner()
    }
}

impl<T> Dependencies for &T
where
    T: ToOwned + ?Sized,
{
    type Owned = <T as ToOwned>::Owned;
    fn to_owned(self) -> Self::Owned {
        self.to_owned()
    }
}

impl<T> Dependencies for Option<&T>
where
    T: 'static + PartialEq + Clone,
{
    type Owned = Option<T>;
    fn to_owned(self) -> Self::Owned {
        self.cloned()
    }
}

impl Dependencies for () {
    type Owned = ();
    fn to_owned(self) {}
}

pub trait TrackEqTuple {
    fn track_eq(&self, ctx: &ComponentCtx) -> bool;
}

impl<T> TrackEqTuple for Sig<'_, T> {
    fn track_eq(&self, _ctx: &ComponentCtx) -> bool {
        self.is_updated()
    }
}

impl<T> TrackEqTuple for &T
where
    T: 'static + PartialEq + Clone,
{
    fn track_eq(&self, ctx: &ComponentCtx) -> bool {
        let sig = ctx.track_eq(*self);
        sig.is_updated()
    }
}

impl<T> TrackEqTuple for Option<&T>
where
    T: 'static + PartialEq + Clone,
{
    fn track_eq(&self, ctx: &ComponentCtx) -> bool {
        let sig =
            ctx.track_eq_custom::<_, Option<T>>(self, |t| t.cloned(), |t, p| *t == p.as_ref());
        sig.is_updated()
    }
}

impl TrackEqTuple for () {
    fn track_eq(&self, ctx: &ComponentCtx) -> bool {
        let sig = ctx.track_eq(self);
        sig.is_updated()
    }
}

macro_rules! track_eq_tuple_impl {
    (
        $(
            ($
                ($T:ident),
            *),
        )*
    ) => {
        $(
            impl<
                $($T: TrackEqTuple,)*
            > TrackEqTuple for ($($T,)*)
            {
                #[allow(non_snake_case)]
                fn track_eq(&self, ctx: &ComponentCtx) -> bool {
                    let ($($T,)*) = self;
                    $(
                        let $T = $T.track_eq(ctx);
                    )*
                    $($T &&)* true
                }
            }
        )*
    };
}

track_eq_tuple_impl!(
    (T0),
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
);

macro_rules! dependencies_impl {
    (
        $(
            ($
                ($T:ident),
            *),
        )*
    ) => {
        $(
            impl<
                $($T: Dependencies,)*
            > Dependencies for ($($T,)*)
            {
                type Owned = ($($T::Owned,)*);
                #[allow(non_snake_case)]
                fn to_owned(self) -> Self::Owned {
                    let ($($T,)*) = self;
                    ($($T.to_owned(),)*)
                }
            }
        )*
    };
}

dependencies_impl!(
    (T0),
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_dependencies_should_be_string() {
        let str_ref = "hello";
        let _cloned: String = Dependencies::to_owned(str_ref);
    }

    #[test]
    fn sig_dependencies_should_be_inner() {
        let world = World::init(Instant::now);
        let sig = Sig::new(&1, SigId::Atom { index: 0 }, &world);
        let _cloned: i32 = sig.to_owned();
    }
}
