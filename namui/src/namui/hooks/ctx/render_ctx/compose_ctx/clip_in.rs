use super::*;

impl<'a> ClipIn for &'a mut ComposeCtx {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        self.clippings.clip_in(xy)
    }
}
