use super::render::Render;
use std::{any::Any, fmt::Debug};

pub trait AnyEqual {
    fn as_any(&self) -> &dyn Any;
    fn equals(&self, _: &dyn Component) -> bool;
    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<S: 'static + PartialEq + Debug> AnyEqual for S {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Component) -> bool {
        other
            .as_any()
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub trait WireClosures {
    fn wire_closures(&self, to: &dyn Component);
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait ComponentProps: AnyEqual {
    fn render(&self, render: Render) -> Render;
}

pub trait Component: ComponentProps + WireClosures {
    fn add_to_render(self, render: &mut Render)
    where
        Self: Sized + 'static,
    {
        render.add_component(self);
    }
}

impl PartialEq for dyn Component {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}

impl<T0, T1> WireClosures for (T0, T1)
where
    T0: WireClosures + 'static,
    T1: WireClosures + 'static,
{
    fn wire_closures(&self, _to: &dyn Component) {
        todo!()
    }
}

impl<T0, T1> ComponentProps for (T0, T1)
where
    T0: Clone + Debug + Any + PartialEq + ComponentProps,
    T1: Clone + Debug + Any + PartialEq + ComponentProps,
{
    fn render(&self, _render: Render) -> Render {
        unreachable!()
    }
}

impl<T0, T1> Component for (T0, T1)
where
    T0: Clone + Debug + Any + PartialEq + Component,
    T1: Clone + Debug + Any + PartialEq + Component,
{
    fn add_to_render(self, render: &mut Render)
    where
        Self: Sized + 'static,
    {
        self.0.add_to_render(render);
        self.1.add_to_render(render);
    }
}
