use super::*;
use crate::*;

pub struct AddingCtx {
    matrix: Matrix3x3,
    tree_ctx: Arc<TreeContext>,
    direct_children: Vec<RenderingTree>,
    push_children_here_on_drop: Arc<Mutex<Vec<RenderingTree>>>,
}

impl Drop for AddingCtx {
    fn drop(&mut self) {
        let mut push_children_here_on_drop = self.push_children_here_on_drop.lock().unwrap();
        push_children_here_on_drop.append(&mut self.direct_children);
    }
}

impl<'a> AddingCtx {
    pub(crate) fn new(
        tree_ctx: Arc<TreeContext>,
        push_children_here_on_drop: Arc<Mutex<Vec<RenderingTree>>>,
        matrix: Matrix3x3,
    ) -> Self {
        Self {
            matrix,
            tree_ctx,
            direct_children: Default::default(),
            push_children_here_on_drop,
        }
    }
    pub fn add(
        &'a mut self,
        add: impl Component, // Name 'add' is to prevent showing 'child' text on rust-analyzer with vscode
    ) -> &'a mut Self {
        let child = self.tree_ctx.render(add, None);
        self.direct_children.push(child);
        self
    }
    pub fn add_with_instance(
        &'a mut self,
        component: impl Component,
        instance: Arc<ComponentInstance>,
    ) -> &'a mut Self {
        let child = self.tree_ctx.render(component, Some(instance));
        self.direct_children.push(child);
        self
    }

    pub fn on_mouse_down_in(&self, on_mouse_down_in: impl FnOnce(crate::MouseEvent)) -> &Self {
        self.on_mouse_internal(on_mouse_down_in, Some(MouseEventType::Down));

        self
    }

    pub fn on_mouse(&self, on_mouse: impl FnOnce(crate::MouseEvent)) -> &Self {
        self.on_mouse_internal(on_mouse, None);

        self
    }
    fn on_mouse_internal(
        &self,
        on_mouse: impl FnOnce(crate::MouseEvent),
        specific_event_type: Option<MouseEventType>,
    ) {
        with_web_event(|event| {
            let (event_type, event) = match event {
                web::WebEvent::MouseDown { event } => (MouseEventType::Down, event),
                web::WebEvent::MouseMove { event } => (MouseEventType::Move, event),
                web::WebEvent::MouseUp { event } => (MouseEventType::Up, event),
                _ => return,
            };

            if specific_event_type.is_some_and(|x| x != event_type) {
                return;
            }

            let local_xy = self.to_local_xy(event.xy);

            if !self.is_local_xy_in(local_xy) {
                return;
            }

            on_mouse(crate::MouseEvent {
                local_xy,
                global_xy: event.xy,
                pressing_buttons: event.pressing_buttons.clone(),
                button: event.button,
                event_type,
                is_stop_propagation: Default::default(), // TODO
            });
        });
    }

    pub fn on_wheel(&self, on_wheel: impl Fn(WheelEvent)) -> &Self {
        with_web_event(|event| {
            if let web::WebEvent::Wheel { event } = event {
                let local_xy = self.to_local_xy(event.mouse_xy);

                if !self.is_local_xy_in(local_xy) {
                    return;
                }

                on_wheel(WheelEvent {
                    delta_xy: event.delta_xy,
                    mouse_local_xy: local_xy,
                    is_stop_propagation: Default::default(), // TODO
                })
            }
        });

        self
    }

    fn is_local_xy_in(&self, local_xy: Xy<Px>) -> bool {
        self.direct_children.iter().any(|x| {
            x.is_xy_in(
                local_xy,
                &[
            // TODO: Ancestors for clipping test
        ],
            )
        })
    }

    fn to_local_xy(&self, xy: Xy<Px>) -> Xy<Px> {
        self.matrix.transform_xy(xy)
    }
}
