use super::*;

pub fn attach_event<'a>(
    component: impl Component + 'a,
    attach_event_build: impl FnOnce(&mut AttachEventBuilder),
) -> AttachEvent<'a> {
    let mut attach_event_builder = AttachEventBuilder::default();
    attach_event_build(&mut attach_event_builder);
    AttachEvent {
        attach_event_builder,
        component: Box::new(component),
    }
}

#[derive(Debug)]
pub struct AttachEvent<'a> {
    attach_event_builder: AttachEventBuilder,
    component: Box<dyn Component + 'a>,
}
impl StaticType for AttachEvent<'_> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<AttachEvent>()
    }
}

impl<'a> Component for AttachEvent<'a> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) {
        let &Self {
            ref attach_event_builder,
            ref component,
        } = self;
        let attach_event_builder = attach_event_builder.clone();

        use_render_with_rendering_tree(
            move |ctx| {
                ctx.add(component.as_ref());
            },
            move |children| {
                RenderingTree::Children(children).attach_event(|builder| {
                    *builder = attach_event_builder.inner.clone();
                })
            },
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct AttachEventBuilder {
    inner: crate::AttachEventBuilder,
}

impl AttachEventBuilder {
    pub fn on_mouse_move_in(
        &mut self,
        on_mouse_move_in: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_move_in(move |mouse_event| {
            on_mouse_move_in.call(mouse_event);
        });
        self
    }

    pub fn on_mouse_move_out(
        &mut self,
        on_mouse_move_out: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_move_out(move |event| {
            on_mouse_move_out.call(event);
        });
        self
    }

    pub fn on_mouse_down_in(
        &mut self,
        on_mouse_down_in: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_down_in(move |event| {
            on_mouse_down_in.call(event);
        });
        self
    }

    pub fn on_mouse_down_out(
        &mut self,
        on_mouse_down_out: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_down_out(move |event| {
            on_mouse_down_out.call(event);
        });
        self
    }

    pub fn on_mouse_up_in(
        &mut self,
        on_mouse_up_in: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_up_in(move |event| {
            on_mouse_up_in.call(event);
        });
        self
    }

    pub fn on_mouse_up_out(
        &mut self,
        on_mouse_up_out: EventCallbackWithParam<crate::MouseEvent>,
    ) -> &mut Self {
        self.inner.on_mouse_up_out(move |event| {
            on_mouse_up_out.call(event);
        });
        self
    }

    pub fn on_mouse(&mut self, on_mouse: EventCallbackWithParam<crate::MouseEvent>) -> &mut Self {
        self.inner.on_mouse(move |event| {
            on_mouse.call(event);
        });
        self
    }

    pub fn on_wheel(&mut self, on_wheel: EventCallbackWithParam<crate::WheelEvent>) -> &mut Self {
        self.inner.on_wheel(move |event| {
            on_wheel.call(event);
        });
        self
    }

    pub fn on_key_down(
        &mut self,
        on_key_down: EventCallbackWithParam<crate::KeyboardEvent>,
    ) -> &mut Self {
        self.inner.on_key_down(move |event| {
            on_key_down.call(event);
        });
        self
    }

    pub fn on_key_up(
        &mut self,
        on_key_up: EventCallbackWithParam<crate::KeyboardEvent>,
    ) -> &mut Self {
        self.inner.on_key_up(move |event| {
            on_key_up.call(event);
        });
        self
    }

    pub fn on_file_drop(
        &mut self,
        on_file_drop: EventCallbackWithParam<crate::FileDropEvent>,
    ) -> &mut Self {
        self.inner.on_file_drop(move |event| {
            on_file_drop.call(event);
        });
        self
    }
}
