use namui::*;

type UnitSig<'a> = Sig<'a, (), ()>;

pub trait Dependencies<'a> {
    fn changed(&self, ctx: &'a RenderCtx) -> UnitSig<'a>;
}

pub trait StaticOwned {
    type Owned: 'static + Send;
    fn owned(&self) -> Self::Owned;
}

impl<'a> Dependencies<'a> for () {
    fn changed(&self, ctx: &'a RenderCtx) -> UnitSig<'a> {
        ctx.track_eq(&()).map2(|_| ())
    }
}
impl StaticOwned for () {
    type Owned = ();
    fn owned(&self) {}
}

impl<'a, T> Dependencies<'a> for &'a T
where
    T: 'static + PartialEq + Send,
    &'a T: ToOwned<Owned = T>,
{
    fn changed(&self, ctx: &'a RenderCtx) -> UnitSig<'a> {
        ctx.track_eq2(self, |a: &T, b| a.eq(*b), |target| target.to_owned())
            .map2(|_| ())
    }
}
impl<'a, T> StaticOwned for &'a T
where
    T: 'static + Send + Clone,
{
    type Owned = T;
    fn owned(&self) -> T {
        self.to_owned().clone()
    }
}

impl<'a, T> Dependencies<'a> for Option<&T>
where
    T: 'static + PartialEq + Clone + Send,
{
    fn changed(&self, ctx: &'a RenderCtx) -> UnitSig<'a> {
        ctx.track_eq2::<Option<T>, _>(
            self,
            |a, b| match (a, b) {
                (Some(a), Some(b)) => a.eq(b),
                (None, None) => true,
                _ => false,
            },
            |b| b.cloned(),
        )
        .map2(|_| ())
    }
}
impl<T> StaticOwned for Option<&T>
where
    T: 'static + Clone + Send,
{
    type Owned = Option<T>;
    fn owned(&self) -> Option<T> {
        self.cloned()
    }
}
