use crate::{app::game::*, ecs};
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

    fn get_position(&self, esc_app: &ecs::App, rendering_context: &RenderingContext) -> Position {
        match &self.subject {
            CameraSubject::Object { id } => esc_app
                .entities()
                .find(|entity| entity.id() == *id)
                .expect("failed to find entity")
                .get_component::<&Mover>()
                .unwrap()
                .get_position(rendering_context.current_time),
            CameraSubject::Position { position } => position.clone(),
        }
    }

    pub fn render(
        &self,
        esc_app: &ecs::App,
        rendering_context: &RenderingContext,
        rendering_tree: RenderingTree,
    ) -> namui::RenderingTree {
        let position = self.get_position(esc_app, rendering_context);
        let screen_center = (rendering_context.screen_size * 0.5).as_xy();
        let offset = (screen_center - position).as_px(rendering_context.px_per_tile);
        translate(offset.x, offset.y, rendering_tree)
    }

    pub fn get_screen(
        &self,
        esc_app: &ecs::App,
        rendering_context: &RenderingContext,
    ) -> Rect<Tile> {
        let position = self.get_position(esc_app, rendering_context);
        let screen_center = (rendering_context.screen_size * 0.5).as_xy();
        Rect::from_xy_wh(position - screen_center, rendering_context.screen_size)
    }

    pub fn get_in_screen_object_list<'a>(
        &self,
        esc_app: &'a ecs::App,
        rendering_context: &RenderingContext,
    ) -> Vec<(&'a ecs::Entity, (&'a Renderer, &'a Mover))> {
        let screen = self.get_screen(esc_app, rendering_context);
        esc_app
            .query_entities::<(&Renderer, &Mover)>()
            .into_iter()
            .filter(|(_, (renderer, mover))| {
                let visual_area =
                    renderer.visual_rect + mover.get_position(rendering_context.current_time);

                visual_area.intersect(screen).is_some()
            })
            .collect()
    }
}
