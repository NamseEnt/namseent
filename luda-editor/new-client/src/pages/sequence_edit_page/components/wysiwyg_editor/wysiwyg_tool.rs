use super::*;

#[namui::component]
pub struct WysiwygTool<'a> {
    pub graphic_dest_rect: Rect<Px>,
    pub original_graphic_size: Wh<Px>,
    pub graphic_index: Uuid,
    pub graphic: &'a ScreenGraphic,
    pub dragging: &'a Option<Dragging>,
    pub wh: Wh<Px>,
    pub on_event: &'a dyn Fn(Event),
}

pub enum Event {
    Mover { event: mover::Event },
    Resizer { event: resizer::Event },
    Rotator { event: rotator::Event },
}

impl Component for WysiwygTool<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            graphic_dest_rect,
            original_graphic_size,
            graphic_index,
            graphic,
            dragging,
            wh,
            on_event,
        } = self;

        ctx.component(Resizer {
            rect: graphic_dest_rect,
            dragging_context: if let Some(Dragging::Resizer { context }) = self.dragging.as_ref() {
                Some(*context)
            } else {
                None
            },
            container_size: wh,
            image_size: calculate_graphic_wh_on_screen(
                original_graphic_size,
                wh,
                graphic.circumscribed(),
            ),
            graphic_index,
            on_event: Box::new(|event| on_event(Event::Resizer { event })),
        });

        let on_rotator_event = |event| on_event(Event::Rotator { event });
        ctx.component(Rotator {
            rect: graphic_dest_rect,
            dragging_context: if let Some(Dragging::Rotator { context }) = self.dragging.as_ref() {
                Some(*context)
            } else {
                None
            },
            graphic_index,
            on_event: &on_rotator_event,
        });

        ctx.component(Mover {
            image_dest_rect: graphic_dest_rect,
            dragging: dragging.clone(),
            container_wh: wh,
            on_event: Box::new(|event| on_event(Event::Mover { event })),
        });

        
    }
}
