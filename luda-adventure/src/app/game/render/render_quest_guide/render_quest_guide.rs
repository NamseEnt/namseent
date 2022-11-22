use crate::{
    app::game::*,
    component::{PlayerCharacter, Positioner, Renderer},
};
use namui::prelude::*;

impl Game {
    pub fn render_quest_guide(&self, rendering_context: &RenderingContext) -> RenderingTree {
        let quest_entity_list = self.state.quest.get_quest_entity_list(&self.ecs_app);
        let character_visual_rect = self.character_visual_rect(rendering_context);
        render([
            render_guide_icon(&quest_entity_list, rendering_context),
            render_guide_arrow(character_visual_rect, &quest_entity_list, rendering_context),
        ])
    }

    fn character_visual_rect(&self, rendering_context: &RenderingContext) -> Option<Rect<Tile>> {
        self.ecs_app
            .query_entities::<(&PlayerCharacter, &Renderer, &Positioner)>()
            .first()
            .map(|(_entity, (_player_character, renderer, positioner))| {
                renderer.visual_rect()
                    + positioner.xy_with_interpolation(rendering_context.interpolation_progress)
            })
    }
}
