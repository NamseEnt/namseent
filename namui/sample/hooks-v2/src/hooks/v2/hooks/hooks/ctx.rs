use super::*;
use std::sync::OnceLock;

static mut CTX: OnceLock<Option<Context>> = OnceLock::new();

pub(crate) fn init() {
    unsafe {
        let _ = CTX.set(None);
    }
}

pub(crate) fn set_up_before_render(
    context_for: ContextFor,
    component_instance: Arc<ComponentInstance>,
) {
    let ctx = unsafe { CTX.get_mut() }.unwrap();

    let prev = ctx.replace(Context::new(context_for, component_instance));
    assert_eq!(prev.is_none(), true);
}

pub(crate) fn take_ctx_and_clear_up() -> Context {
    let ctx = unsafe { CTX.get_mut() }.unwrap().take().unwrap();
    ctx.instance
        .is_first_render
        .store(false, std::sync::atomic::Ordering::SeqCst);
    ctx
}

pub(crate) fn ctx() -> &'static Context {
    unsafe { CTX.get() }.unwrap().as_ref().unwrap()
}
