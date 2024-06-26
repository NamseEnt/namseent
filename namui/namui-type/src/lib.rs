mod log;
mod types;

pub use auto_ops;
use derive_macro::type_derives;
pub use log::*;
use ordered_float::OrderedFloat;
pub use postcard;
pub use types::*;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log(format!($($arg)*));
    }}
}
