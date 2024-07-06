use namui::*;

pub trait Dependencies<'a> {
    fn changed(&self, ctx: &'a RenderCtx) -> Sig<'a, (), ()>;
}

impl<'a> Dependencies<'a> for () {
    fn changed(&self, ctx: &'a RenderCtx) -> Sig<'a, (), ()> {
        ctx.track_eq(&()).map2(|_| ())
    }
}

impl<'a, T> Dependencies<'a> for &'a T
where
    T: 'static + PartialEq + ToOwned<Owned = T>,
{
    fn changed(&self, ctx: &'a RenderCtx) -> Sig<'a, (), ()> {
        let sig: Sig<T, _> = ctx.track_eq2(
            self,
            |a: &T, b| a.eq(*b),
            |target| target.to_owned().to_owned(),
        );
        sig.map2(move |_| ())
    }
}

impl<'a, T> Dependencies<'a> for Option<&T>
where
    T: 'static + PartialEq + Clone,
{
    fn changed(&self, ctx: &'a RenderCtx) -> Sig<'a, (), ()> {
        let sig = ctx.track_eq2::<Option<T>, _>(
            self,
            |a, b| match (a, b) {
                (Some(a), Some(b)) => a.eq(b),
                (None, None) => true,
                _ => false,
            },
            |b| b.cloned(),
        );
        sig.map2(|_| ())
    }
}
