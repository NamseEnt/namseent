use super::*;
use std::sync::OnceLock;

static mut CTX_STACK: OnceLock<Vec<Context>> = OnceLock::new();

pub(crate) fn init() {
    unsafe {
        let _ = CTX_STACK.set(vec![]);
    }
}

pub(crate) fn set_up_before_render(
    context_for: ContextFor,
    component_instance: Arc<ComponentInstance>,
) {
    let ctx_stack = unsafe { CTX_STACK.get_mut() }.unwrap();

    ctx_stack.push(Context::new(context_for, component_instance));
}

pub(crate) fn take_ctx_and_clear_up() -> Context {
    let ctx_stack = unsafe { CTX_STACK.get_mut() }.unwrap();
    let ctx = ctx_stack.pop().unwrap();
    ctx.instance
        .is_first_render
        .store(false, std::sync::atomic::Ordering::SeqCst);
    ctx
}

pub(crate) fn ctx() -> &'static Context {
    unsafe { CTX_STACK.get() }.unwrap().last().unwrap()
}
