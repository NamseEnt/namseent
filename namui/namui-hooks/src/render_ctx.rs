use crate::*;
use std::ops::Deref;
use std::rc::Rc;

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
impl<'a, 'rt> RenderCtx<'a, 'rt> {
    #[deprecated(since = "0.2.0", note = "Use `add` instead")]
    pub fn component(&self, component: impl Component) {
        self.compose_ctx.add(component);
    }
}

// Component
impl<'a, 'rt> RenderCtx<'a, 'rt> {
    pub fn state<T: 'static>(&self, init: impl FnOnce() -> T) -> (Sig<T, &T>, SetState<T>) {
        self.component_ctx.state(init)
    }
    pub fn memo<T: 'static>(&self, func: impl FnOnce() -> T) -> Sig<T, Rc<T>> {
        self.component_ctx.memo(func)
    }
    pub fn track_eq<T: 'static + PartialEq + Clone>(&self, target: &T) -> Sig<T, Rc<T>> {
        self.component_ctx.track_eq(target)
    }
    pub fn effect<CleanUp: Into<EffectCleanUp>>(
        &self,
        title: impl AsRef<str>,
        func: impl FnOnce() -> CleanUp,
    ) {
        self.component_ctx.effect(title, func)
    }
    pub fn interval(&self, title: impl AsRef<str>, interval: Duration, job: impl FnOnce(Duration)) {
        self.component_ctx.interval(title, interval, job)
    }
    pub fn controlled_memo<T: 'static>(
        &self,
        func: impl FnOnce(Option<T>) -> ControlledMemo<T>,
    ) -> Sig<T, Rc<T>> {
        self.component_ctx.controlled_memo(func)
    }
    pub fn init_atom<State: Send + Sync + 'static>(
        &self,
        atom: &'static Atom<State>,
        init: impl Fn() -> State,
    ) -> (Sig<State, &State>, StaticSetState<State>) {
        self.component_ctx.init_atom(atom, init)
    }
    pub fn atom<State: Send + Sync + 'static>(
        &self,
        atom: &'static Atom<State>,
    ) -> (Sig<State, &State>, StaticSetState<State>) {
        self.component_ctx.atom(atom)
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
