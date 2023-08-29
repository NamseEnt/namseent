/// callback!('a, A)
#[macro_export]
macro_rules! callback {
    ($lifetime: lifetime, $param: ty) => {
        // TODO: Make it `&dyn 'a Fn($param)`
        Box<dyn $lifetime + FnOnce($param)>
    };
    ($lifetime: lifetime) => {
        Box<dyn $lifetime + FnOnce()>
    };
}

pub use callback;
