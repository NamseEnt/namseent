use super::*;

impl<'a> ClipIn for &'a RenderCtx {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        global_state::clippings().clip_in(xy)
    }
}
