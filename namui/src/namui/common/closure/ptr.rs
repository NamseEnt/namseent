use std::sync::{Arc, Mutex};

/// This struct's PartialEq always returns true.
pub struct ClosurePtr<Param, Return> {
    inner: Arc<Mutex<Arc<ClosurePtrInner<Param, Return>>>>,
}

unsafe impl<Param, Return> Send for ClosurePtr<Param, Return>
where
    Param: Send,
    Return: Send,
{
}
unsafe impl<Param, Return> Sync for ClosurePtr<Param, Return>
where
    Param: Send,
    Return: Send,
{
}

impl<Param, Return> Clone for ClosurePtr<Param, Return> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Param, Return> std::fmt::Debug for ClosurePtr<Param, Return> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosurePtr")
            .field("inner", &self.inner.lock().unwrap())
            .finish()
    }
}

impl<Param, Return> PartialEq for ClosurePtr<Param, Return> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<Param, Return> ClosurePtr<Param, Return> {
    pub fn new(closure: impl Fn(Param) -> Return + 'static) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Arc::new(ClosurePtrInner::new(Arc::new(
                closure,
            ))))),
        }
    }

    pub fn invoke(&self, param: Param) -> Return {
        { self.inner.lock().unwrap().clone() }.invoke(param)
    }

    pub fn set_closure(&self, closure: impl Fn(Param) -> Return + 'static) {
        *self.inner.lock().unwrap() = Arc::new(ClosurePtrInner::new(Arc::new(closure)));
    }

    pub fn wire_to(&self, to: &ClosurePtr<Param, Return>) {
        *self.inner.lock().unwrap() = Arc::new(ClosurePtrInner::Wire {
            next: to.inner.clone(),
        });
    }

    pub fn wired(&self) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Arc::new(ClosurePtrInner::Wire {
                next: self.inner.clone(),
            }))),
        }
    }
}

pub enum ClosurePtrInner<Param, Return> {
    End {
        closure: Mutex<Arc<dyn Fn(Param) -> Return>>,
    },
    Wire {
        next: Arc<Mutex<Arc<ClosurePtrInner<Param, Return>>>>,
    },
}
impl<Param, Return> ClosurePtrInner<Param, Return> {
    pub fn new(closure_arc: Arc<dyn Fn(Param) -> Return>) -> Self {
        Self::End {
            closure: Mutex::new(closure_arc),
        }
    }

    pub fn invoke(&self, param: Param) -> Return {
        match self {
            ClosurePtrInner::End { closure } => {
                let closure = { closure.lock().unwrap().clone() };
                closure(param)
            }
            ClosurePtrInner::Wire { next } => {
                let next = { next.lock().unwrap().clone() };
                next.invoke(param)
            }
        }
    }
}

impl<Param, Return> std::fmt::Debug for ClosurePtrInner<Param, Return> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClosurePtrInner::End { closure } => f
                .debug_struct("ClosurePtrInner::End")
                .field("closure", &Arc::as_ptr(&closure.lock().unwrap()))
                .finish(),
            ClosurePtrInner::Wire { next } => f
                .debug_struct("ClosurePtrInner::Wire")
                .field("next", &next.lock().unwrap())
                .finish(),
        }
    }
}

impl<Param, Return> PartialEq for ClosurePtrInner<Param, Return> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<F, Param, Return> From<F> for ClosurePtr<Param, Return>
where
    F: Fn(Param) -> Return + 'static,
{
    fn from(func: F) -> Self {
        Self::new(func)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn wired_ptr_should_be_point_same_closure() {
        let a = Arc::new(AtomicUsize::new(0));

        let cptr1 = ClosurePtr::new({
            let a = a.clone();
            move |_| {
                a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });
        let cptr2 = cptr1.wired();
        let cptr3 = cptr2.wired();

        cptr1.invoke(());
        cptr2.invoke(());
        cptr3.invoke(());

        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 3);

        //----------------------

        let b = Arc::new(AtomicUsize::new(0));

        let closure = {
            let b = b.clone();
            move |_| {
                b.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        };

        cptr1.set_closure(closure);

        cptr1.invoke(());
        cptr2.invoke(());
        cptr3.invoke(());

        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 3);
        assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), 3);

        //------------------------

        let c = Arc::new(AtomicUsize::new(0));

        let closure = {
            let c = c.clone();
            move |_| {
                c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        };

        cptr2.set_closure(closure);

        cptr1.invoke(());
        cptr2.invoke(());
        cptr3.invoke(());

        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 3);
        assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), 4);
        assert_eq!(c.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn into_closure_ptr_should_works() {
        fn test<F>(f: F)
        where
            F: Into<ClosurePtr<(), ()>>,
        {
            let cptr = f.into();
            cptr.invoke(());
        }

        let a = Arc::new(AtomicUsize::new(0));
        test({
            let a = a.clone();
            move |_| {
                a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });
        assert_eq!(a.load(std::sync::atomic::Ordering::SeqCst), 1);

        let b = Arc::new(AtomicUsize::new(0));
        test(ClosurePtr::new({
            let b = b.clone();
            move |_| {
                b.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }));
        assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
