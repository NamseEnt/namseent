mod ptr;

pub use ptr::*;

pub fn closure<Param, Return>(
    func: impl Fn(Param) -> Return + 'static,
) -> ClosurePtr<Param, Return> {
    ClosurePtr::new(func)
}
