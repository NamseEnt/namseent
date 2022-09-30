use crate::{app::game::*, ecs};
use namui::prelude::*;

pub struct Camera {
    subject: CameraSubject,
}

#[derive(Clone, Copy)]
pub enum CameraSubject {
    Object { id: Uuid },
    Position { position: Xy<Tile> },
}

impl Camera {
    pub fn new(subject: Option<CameraSubject>) -> Self {
        Self {
            subject: subject.unwrap_or(CameraSubject::Position {
                position: Xy::new(0.tile(), 0.tile()),
            }),
        }
    }

    pub fn update(&mut self, _event: &dyn std::any::Any) {}

    pub fn get_position(&self, esc_app: &ecs::App, time: Time) -> Xy<Tile> {
        match &self.subject {
            CameraSubject::Object { id } => esc_app
                .entities()
                .find(|entity| entity.id() == *id)
                .expect("failed to find entity")
                .get_component::<&Positioner>()
                .unwrap()
                .get_position(time),
            CameraSubject::Position { position } => position.clone(),
        }
    }

    pub fn translate_to_camera_screen(
        &self,
        rendering_context: &RenderingContext,
        rendering_tree: RenderingTree,
    ) -> namui::RenderingTree {
        translate(
            -(rendering_context.px_per_tile * rendering_context.screen_rect.x()),
            -(rendering_context.px_per_tile * rendering_context.screen_rect.y()),
            rendering_tree,
        )
    }

    pub fn get_in_screen_object_list<'a>(
        &self,
        esc_app: &'a ecs::App,
        rendering_context: &RenderingContext,
    ) -> Vec<(&'a ecs::Entity, (&'a Renderer, &'a Positioner))> {
        esc_app
            .query_entities::<(&Renderer, &Positioner)>()
            .into_iter()
            .filter(|(_, (renderer, positioner))| {
                let visual_area =
                    renderer.visual_rect + positioner.get_position(rendering_context.current_time);

                visual_area
                    .intersect(rendering_context.screen_rect)
                    .is_some()
            })
            .collect()
    }
}
