use super::*;

impl ComposeCtx {
    pub fn translate(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();
        let lazy: LazyShared = Default::default();

        self.add_lazy(LazyRenderingTree::Translate {
            xy,
            lazy: lazy.clone(),
        });

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::push_transform_matrix(|matrix| {
                matrix.translate(xy.x.as_f32(), xy.y.as_f32())
            }),
        )
    }
    pub fn absolute(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();
        let lazy: LazyShared = Default::default();
        self.add_lazy(LazyRenderingTree::Absolute {
            xy,
            lazy: lazy.clone(),
        });

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::push_transform_matrix(|matrix| {
                *matrix = TransformMatrix::from_translate(xy.x.as_f32(), xy.y.as_f32())
            }),
        )
    }
    pub fn clip(&mut self, path: crate::Path, clip_op: crate::ClipOp) -> Self {
        let lazy: LazyShared = Default::default();
        self.add_lazy(LazyRenderingTree::Clip {
            path: path.clone(),
            clip_op,
            lazy: lazy.clone(),
        });

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::push_clipping(Clipping {
                path: path.transform(*global_state::matrix()),
                clip_op,
            }),
        )
    }
    pub fn on_top(&mut self) -> Self {
        let lazy: LazyShared = Default::default();
        self.add_lazy(LazyRenderingTree::OnTop { lazy: lazy.clone() });

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::top(),
        )
    }
    pub fn rotate(&mut self, angle: Angle) -> Self {
        let lazy: LazyShared = Default::default();
        self.add_lazy(LazyRenderingTree::Rotate {
            angle,
            lazy: lazy.clone(),
        });
        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::push_transform_matrix(|matrix| matrix.rotate(angle)),
        )
    }
    pub fn scale(&mut self, scale_xy: Xy<f32>) -> Self {
        let lazy: LazyShared = Default::default();
        self.add_lazy(LazyRenderingTree::Scale {
            scale_xy,
            lazy: lazy.clone(),
        });

        ComposeCtx::new(
            self.next_child_key_vec(),
            self.renderer.clone(),
            lazy,
            global_state::push_transform_matrix(|matrix| matrix.scale(scale_xy.x, scale_xy.y)),
        )
    }
    pub fn attach_event(&mut self, on_event: impl FnOnce(Event<'_>)) -> &mut Self {
        let Some(raw_event) = global_state::raw_event() else {
            return self;
        };

        let rendering_tree = {
            let rendering_trees: Vec<_> = std::mem::take(&mut self.lazy_children)
                .into_iter()
                .map(|x| x.get_rendering_tree())
                .collect();
            self.unlazy_children.extend(rendering_trees);
            RenderingTree::Children(self.unlazy_children.clone())
        };

        invoke_on_event(
            global_state::tree_ctx(),
            on_event,
            raw_event,
            global_state::matrix().inverse().unwrap(),
            &rendering_tree,
            global_state::clippings(),
        );

        self
    }
}
