use super::*;

impl ComposeCtx {
    pub fn translate(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();

        ComposeCtx::new(
            self.get_or_create_compose_child(),
            GlobalStatePop::translate(xy),
        )
    }
    pub fn absolute(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();

        ComposeCtx::new(
            self.get_or_create_compose_child(),
            GlobalStatePop::absolute(xy),
        )
    }
    pub fn clip(&mut self, path: crate::Path, clip_op: crate::ClipOp) -> Self {
        ComposeCtx::new(
            self.get_or_create_compose_child(),
            GlobalStatePop::clip(Clipping {
                path: path.transform(*global_state::matrix()),
                clip_op,
            }),
        )
    }
    pub fn on_top(&mut self) -> Self {
        ComposeCtx::new(self.get_or_create_compose_child(), GlobalStatePop::top())
    }
    pub fn rotate(&mut self, angle: Angle) -> Self {
        ComposeCtx::new(
            self.get_or_create_compose_child(),
            GlobalStatePop::rotate(angle),
        )
    }
    pub fn scale(&mut self, scale_xy: Xy<f32>) -> Self {
        ComposeCtx::new(
            self.get_or_create_compose_child(),
            GlobalStatePop::scale(scale_xy),
        )
    }
    pub fn attach_event(&mut self, on_event: impl FnOnce(Event<'_>)) -> &mut Self {
        let Some(raw_event) = global_state::raw_event() else {
            return self;
        };

        invoke_on_event(
            global_state::tree_ctx(),
            on_event,
            raw_event,
            global_state::matrix().inverse().unwrap(),
            global_state::iter_last_rendering_tree(),
            global_state::clippings(),
        );

        self
    }
}
