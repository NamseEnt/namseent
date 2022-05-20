#[macro_use]
mod define_singular_floating_tuple;
mod time;
pub use self::time::*;

// NOTE: Please move type into new file when it has impl.

define_singular_floating_tuple!(PixelSize, f32); // NOTE: `PixelSize` naming is for distinguishing from `PixelColor`.
define_singular_floating_tuple!(Angle, f32);
define_singular_floating_tuple!(OneZero, f32, |value| OneZero(num::clamp(value, 0.0, 1.0)));
