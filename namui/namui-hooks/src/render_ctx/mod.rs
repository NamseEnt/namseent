mod dependencies;

use crate::*;
pub use dependencies::*;
use std::ops::Deref;

pub struct RenderCtx<'a, 'rt> {
    component_ctx: ComponentCtx<'a>,
    compose_ctx: ComposeCtx<'a, 'rt>,
}

impl<'a, 'rt> Deref for RenderCtx<'a, 'rt> {
    type Target = ComposeCtx<'a, 'rt>;

    fn deref(&self) -> &Self::Target {
        &self.compose_ctx
    }
}

// Compose
impl RenderCtx<'_, '_> {
    #[deprecated(since = "0.2.0", note = "Use `add` instead")]
    pub fn component(&self, component: impl Component) {
        self.compose_ctx.add(component);
    }
}

// Component
impl RenderCtx<'_, '_> {
    pub fn state<T: State>(&self, init: impl FnOnce() -> T) -> (Sig<'_, T>, SetState<T>) {
        self.component_ctx.state(init)
    }
    pub fn memo<T: State>(&self, func: impl FnOnce() -> T) -> Sig<'_, T> {
        self.component_ctx.memo(func)
    }
    pub fn track_eq<T: State + PartialEq + Clone>(&self, target: &T) -> Sig<'_, T> {
        self.component_ctx.track_eq(target)
    }
    pub fn track_eq_tuple(&self, track_eq_tuple: &impl TrackEqTuple) -> bool {
        self.component_ctx.track_eq_tuple(track_eq_tuple)
    }
    pub fn effect<CleanUp: Into<EffectCleanUp>>(
        &self,
        title: impl AsRef<str>,
        func: impl FnOnce() -> CleanUp,
    ) {
        self.component_ctx.effect(title, func)
    }
    pub fn async_effect<Fut, Deps>(
        &self,
        _title: impl AsRef<str>,
        deps: Deps,
        future_fn: impl FnOnce(<Deps as Dependencies>::Owned) -> Fut,
    ) where
        Fut: std::future::Future + Send + 'static,
        Fut::Output: Send + 'static,
        Deps: Dependencies + TrackEqTuple,
    {
        if deps.track_eq(&self.component_ctx) {
            let owned = deps.to_owned();
            self.spawn(future_fn(owned))
        }
    }
    pub fn interval(&self, title: impl AsRef<str>, interval: Duration, job: impl FnOnce(Duration)) {
        self.component_ctx.interval(title, interval, job)
    }
    /// Returning `ControlledMemo::Unchanged(value)` will not update sig, but it will return `Sig(value)`.
    pub fn controlled_memo<T: State>(
        &self,
        func: impl FnOnce(Option<T>) -> ControlledMemo<T>,
    ) -> Sig<'_, T> {
        self.component_ctx.controlled_memo(func)
    }
    pub fn init_atom<T: State>(
        &self,
        atom: &'static Atom<T>,
        init: impl Fn() -> T,
    ) -> (Sig<'_, T>, SetState<T>) {
        self.component_ctx.init_atom(atom, init)
    }
    pub fn atom<T: State>(&self, atom: &'static Atom<T>) -> (Sig<'_, T>, SetState<T>) {
        self.component_ctx.atom(atom)
    }
    /// This method just keep JoinHandle to abort when the component is unmounted.
    /// This is not the replacement of `async_effect`, but `tokio::spawn`.
    pub fn spawn<Fut>(&self, future: Fut)
    where
        Fut: std::future::Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        self.component_ctx.spawn(future)
    }
    pub fn is_sig_updated<T>(&self, sig: &Sig<'_, T>) -> bool {
        self.component_ctx.is_sig_updated(&sig.id)
    }
}

pub(crate) fn run<'a>(
    world: &'a World,
    component: impl Component,
    composer: &'a Composer,
    instance: &'a Instance,
    full_stack: CowFullStack<'a>,
) -> RenderingTree {
    let rt_container = RtContainer::new();

    {
        let ctx = RenderCtx {
            component_ctx: ComponentCtx::new(world, instance),
            compose_ctx: ComposeCtx::new(world, composer, &rt_container, full_stack),
        };

        component.render(&ctx);
    }

    rt_container.into()
}
