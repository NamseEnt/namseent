/// only for the method with RenderCtx.
/// ex) ctx.atom() or ctx.state()
mod atom;
mod effect;
mod memo;
mod state;
mod track_eq;

use super::*;
pub use atom::*;
pub(crate) use effect::*;
pub(crate) use memo::*;
pub use state::*;
pub(crate) use track_eq::*;

fn update_or_push<T>(vector: &mut Vec<T>, index: usize, value: T) {
    if let Some(prev) = vector.get_mut(index) {
        *prev = value;
    } else {
        assert_eq!(vector.len(), index);
        vector.insert(index, value);
    }
}
