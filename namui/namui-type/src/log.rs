type LogFn = Box<dyn Fn(&str)>;
static mut LOG_FN: Option<LogFn> = None;

pub fn set_log(log: impl Fn(&str) + 'static) {
    unsafe {
        LOG_FN = Some(Box::new(log));
    }
}

pub fn log(content: impl AsRef<str>) {
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(log) = LOG_FN.as_ref() {
            log(content.as_ref());
        }
    }
}
