use crate::app::game::{GameObject, Position, PositionExt, RenderingContext, Tile, TileExt};
use namui::prelude::*;

pub struct Camera {
    subject: CameraSubject,
}

#[derive(Clone, Copy)]
pub enum CameraSubject {
    Object { id: Uuid },
    Position { position: Position },
}

pub enum Event {
    // SetSubject { subject: CameraSubject },
}

impl Camera {
    pub fn new(subject: Option<CameraSubject>) -> Self {
        Self {
            subject: subject.unwrap_or(CameraSubject::Position {
                position: Position::new(0.tile(), 0.tile()),
            }),
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(_event) = event.downcast_ref::<Event>() {
            // match event {
            //     Event::SetSubject { subject } => self.subject = subject.clone(),
            // }
        }
    }

    fn get_position(
        &self,
        object_list: &Vec<Box<dyn GameObject>>,
        rendering_context: &RenderingContext,
    ) -> Position {
        match &self.subject {
            CameraSubject::Object { id } => object_list
                .iter()
                .find(|object| object.get_id() == *id)
                .expect("failed to find object")
                .get_position(rendering_context.current_time),
            CameraSubject::Position { position } => position.clone(),
        }
    }

    pub fn render(
        &self,
        object_list: &Vec<Box<dyn GameObject>>,
        rendering_context: &RenderingContext,
        rendering_tree: RenderingTree,
    ) -> namui::RenderingTree {
        let position = self.get_position(object_list, rendering_context);
        let screen_center = (rendering_context.screen_size * 0.5).as_xy();
        let offset = (screen_center - position).as_px(rendering_context.px_per_tile);
        translate(offset.x, offset.y, rendering_tree)
    }

    pub fn get_screen(
        &self,
        object_list: &Vec<Box<dyn GameObject>>,
        rendering_context: &RenderingContext,
    ) -> Rect<Tile> {
        let position = self.get_position(object_list, rendering_context);
        let screen_center = (rendering_context.screen_size * 0.5).as_xy();
        Rect::from_xy_wh(position - screen_center, rendering_context.screen_size)
    }

    pub fn get_in_screen_object_list<'a>(
        &self,
        object_list: &'a Vec<Box<dyn GameObject>>,
        rendering_context: &RenderingContext,
    ) -> Vec<&'a Box<dyn GameObject>> {
        let screen = self.get_screen(object_list, rendering_context);
        object_list
            .iter()
            .filter(|object| {
                object
                    .get_visual_area(rendering_context.current_time)
                    .intersect(screen)
                    .is_some()
            })
            .collect()
    }
}
