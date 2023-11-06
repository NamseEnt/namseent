use crate::*;

#[derive(Debug, Clone)]
pub struct Clipping {
    pub path: Path,
    pub clip_op: ClipOp,
}

pub trait ClipIn {
    /// Returns true if the given xy is in the clip, so it should be kept.
    fn clip_in(&self, xy: Xy<Px>) -> bool;
}

impl<'a> ClipIn for &'a Vec<Clipping> {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        self.iter().all(|clipping| clipping.clip_in(xy))
    }
}

impl ClipIn for Vec<Clipping> {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        (&self).clip_in(xy)
    }
}

impl ClipIn for Clipping {
    fn clip_in(&self, xy: Xy<Px>) -> bool {
        let path_contains = crate::system::skia::path_contains_xy(&self.path, None, xy);
        match self.clip_op {
            ClipOp::Intersect => path_contains,
            ClipOp::Difference => !path_contains,
        }
    }
}
