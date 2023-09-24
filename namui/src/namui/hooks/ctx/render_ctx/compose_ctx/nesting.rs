use super::*;

impl ComposeCtx {
    pub fn translate(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        self.add_lazy(LazyRenderingTree::Translate {
            xy,
            lazy: lazy.clone(),
        });

        let matrix = self.matrix * Matrix3x3::from_translate(xy.x.as_f32(), xy.y.as_f32());
        ComposeCtx::new(
            self.tree_ctx.clone(),
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            lazy,
            self.raw_event.clone(),
            self.clippings.clone(),
        )
    }
    pub fn absolute(&mut self, xy: impl IntoXyPx) -> Self {
        let xy = xy.into_xy_px();
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        self.add_lazy(LazyRenderingTree::Absolute {
            xy,
            lazy: lazy.clone(),
        });

        let matrix = Matrix3x3::from_translate(xy.x.as_f32(), xy.y.as_f32());
        ComposeCtx::new(
            self.tree_ctx.clone(),
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            lazy,
            self.raw_event.clone(),
            self.clippings.clone(),
        )
    }
    pub fn clip(&mut self, path: crate::Path, clip_op: crate::ClipOp) -> Self {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        self.add_lazy(LazyRenderingTree::Clip {
            path: path.clone(),
            clip_op,
            lazy: lazy.clone(),
        });

        let clippings = {
            let mut clippings = self.clippings.clone();
            let path = path.transform(self.matrix);
            clippings.push(Clipping { path, clip_op });
            clippings
        };

        ComposeCtx::new(
            self.tree_ctx.clone(),
            self.next_child_key_vec(),
            self.matrix,
            self.renderer.clone(),
            lazy,
            self.raw_event.clone(),
            clippings,
        )
    }
    pub fn on_top(&mut self) -> Self {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        self.add_lazy(LazyRenderingTree::OnTop { lazy: lazy.clone() });

        let matrix = self.matrix;
        ComposeCtx::new(
            self.tree_ctx.clone(),
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            lazy,
            self.raw_event.clone(),
            vec![],
        )
    }
    pub fn rotate(&mut self, angle: Angle) -> Self {
        let lazy: Arc<Mutex<Option<LazyRenderingTree>>> = Default::default();
        self.add_lazy(LazyRenderingTree::Rotate {
            angle,
            lazy: lazy.clone(),
        });

        let matrix = self.matrix * Matrix3x3::from_rotate(angle);
        ComposeCtx::new(
            self.tree_ctx.clone(),
            self.next_child_key_vec(),
            matrix,
            self.renderer.clone(),
            lazy,
            self.raw_event.clone(),
            self.clippings.clone(),
        )
    }
    pub fn attach_event(&mut self, on_event: impl FnOnce(Event<'_>)) -> &mut Self {
        if let Some(raw_event) = self.raw_event.lock().unwrap().clone() {
            let rendering_tree = {
                let rendering_trees: Vec<_> = std::mem::take(&mut self.lazy_children)
                    .into_iter()
                    .map(|x| x.lock().unwrap().take().unwrap().into_rendering_tree())
                    .collect();
                self.unlazy_children.extend(rendering_trees);
                RenderingTree::Children(self.unlazy_children.clone())
            };
            invoke_on_event(
                &self.tree_ctx,
                on_event,
                &raw_event,
                self.matrix.inverse().unwrap(),
                &rendering_tree,
                &self.clippings,
            );
        }

        self
    }
}
