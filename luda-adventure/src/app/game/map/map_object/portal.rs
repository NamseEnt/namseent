use crate::{
    app::game::{Tile, TileExt},
    component::{Interactor, Positioner, RenderType, Renderer, Sprite},
};
use namui::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portal {
    pub xy: Xy<Tile>,
    pub destination_map_name: String,
    pub destination_character_xy: Xy<Tile>,
}

impl Portal {
    pub fn create_entity(&self, app: &mut crate::ecs::App) {
        app.new_entity()
            .add_component(Positioner::new_with_xy(self.xy))
            .add_component(Interactor {
                kind: crate::app::game::InteractionKind::MapTeleport {
                    map_name: self.destination_map_name.clone(),
                    player_xy: self.destination_character_xy,
                },
            })
            .add_component(Renderer::new(
                0,
                RenderType::Sprite(Sprite {
                    visual_rect: Rect::Xywh {
                        x: -1.tile(),
                        y: -2.tile(),
                        width: 2.tile(),
                        height: 3.tile(),
                    },
                    image_url: Url::parse("bundle:image/portal.png").unwrap(),
                }),
            ));
    }
}
