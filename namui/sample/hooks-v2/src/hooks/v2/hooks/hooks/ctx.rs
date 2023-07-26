use super::*;
use std::sync::OnceLock;

static mut CTX: OnceLock<Option<Context>> = OnceLock::new();

pub(crate) fn init() {
    unsafe {
        let _ = CTX.set(None);
    }
}

pub(crate) fn set_up_before_render(component_instance: Arc<ComponentInstance>) {
    let ctx = unsafe { CTX.get_mut() }.unwrap();

    ctx.replace(Context::new(component_instance));
}

pub(crate) fn clear_up_before_render() {
    unsafe { CTX.get_mut() }.unwrap().take();
}

pub(crate) fn ctx() -> &'static Context {
    unsafe { CTX.get() }.unwrap().as_ref().unwrap()
}
