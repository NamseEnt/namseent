static mut LOG: Option<Box<dyn Fn(&str)>> = None;

pub fn set_log(log: impl Fn(&str) + 'static) {
    unsafe {
        LOG = Some(Box::new(log));
    }
}

pub fn log(content: impl AsRef<str>) {
    unsafe {
        if let Some(log) = &LOG {
            log(content.as_ref());
        }
    }
}
