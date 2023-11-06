use std::fmt::Debug;

pub trait AnyPartialEq {
    fn equals(&self, other: &dyn AnyPartialEq) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Debug for dyn AnyPartialEq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}

impl Debug for dyn AnyPartialEq + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}

impl<T: 'static + std::any::Any + PartialEq + Debug> AnyPartialEq for T {
    fn equals(&self, other: &dyn AnyPartialEq) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct DynAnyClonePartialEq<'a> {
    inner: &'a dyn AnyPartialEq,
}
impl<'a> PartialEq for DynAnyClonePartialEq<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.equals(other.inner)
    }
}
impl<'a> Debug for DynAnyClonePartialEq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.debug(f)
    }
}
