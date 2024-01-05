use super::*;

impl ClipIn for &RenderCtx {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        self.inner().clippings.clip_in(xy)
    }
}
