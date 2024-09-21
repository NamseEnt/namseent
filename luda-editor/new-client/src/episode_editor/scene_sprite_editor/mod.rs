mod position_tool;
mod scene_sprite_list;
mod size_tool;
mod sprite_select_tool;

use luda_rpc::{AssetDoc, Circumcircle, Scene, SceneSprite};
use namui::*;
use namui_prebuilt::*;
use std::collections::HashMap;

pub struct SceneSpriteEditor<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub update_scene: &'a dyn Fn(Scene),
    pub asset_docs: Sig<'a, HashMap<String, AssetDoc>>,
}

impl Component for SceneSpriteEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            update_scene,
            asset_docs,
        } = self;

        let (selected_scene_sprite_index, set_selected_scene_sprite_index) = ctx.state(|| None);

        let scene_sprites = &scene.scene_sprites;
        let selected_scene_sprite = selected_scene_sprite_index
            .as_ref()
            .and_then(|index: usize| scene_sprites.get(index));

        let remove_scene_sprite = &|index: usize| {
            let mut scene = scene.clone();
            scene.scene_sprites.remove(index);
            update_scene(scene);
        };

        let add_new_scene_sprite = &|| {
            let mut scene = scene.clone();
            scene.scene_sprites.push(SceneSprite {
                sprite_id: None,
                circumcircle: Circumcircle {
                    xy: Xy::single(50.percent()),
                    radius: 50.percent(),
                },
                part_option_selections: HashMap::new(),
            });
            update_scene(scene);
        };

        let move_scene_sprite_up_down = &|index: usize, upward: bool| {
            let mut scene = scene.clone();
            let target_index = match upward {
                true => index.checked_sub(1),
                false => index.checked_add(1),
            };
            if let Some(target_index) = target_index {
                scene.scene_sprites.swap(index, target_index);
                update_scene(scene);
            }
        };

        let select_scene_sprite_index = &|index: usize| {
            set_selected_scene_sprite_index.set(Some(index));
        };

        let select_part_option =
            &|part_name: &str, part_option_name: &str, is_single_select: bool| {
                let Some(index) = *selected_scene_sprite_index else {
                    return;
                };
                let mut scene = scene.clone();
                let Some(scene_sprite) = scene.scene_sprites.get_mut(index) else {
                    return;
                };
                let Some(part_option_selection) =
                    scene_sprite.part_option_selections.get_mut(part_name)
                else {
                    return;
                };

                let already_selected = part_option_selection.contains(part_option_name);
                if is_single_select {
                    part_option_selection.clear();
                    part_option_selection.insert(part_option_name.to_string());
                } else if already_selected {
                    part_option_selection.remove(part_option_name);
                } else {
                    part_option_selection.insert(part_option_name.to_string());
                }

                update_scene(scene);
            };

        let on_change_position = &|position: Xy<Percent>| {
            let Some(index) = *selected_scene_sprite_index else {
                return;
            };
            let mut scene = scene.clone();
            let Some(scene_sprite) = scene.scene_sprites.get_mut(index) else {
                return;
            };
            scene_sprite.circumcircle.xy = position;
            update_scene(scene);
        };

        let on_change_size_radius = &|size_radius: Percent| {
            let Some(index) = *selected_scene_sprite_index else {
                return;
            };
            let mut scene = scene.clone();
            let Some(scene_sprite) = scene.scene_sprites.get_mut(index) else {
                return;
            };
            scene_sprite.circumcircle.radius = size_radius;
            update_scene(scene);
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(320.px(), {
                    |wh, ctx| {
                        ctx.add(scene_sprite_list::SceneSpriteList {
                            wh,
                            scene_sprites,
                            asset_docs: &asset_docs,
                            remove_scene_sprite,
                            add_new_scene_sprite,
                            move_scene_sprite_up_down,
                            select_scene_sprite_index,
                            selected_scene_sprite_index: *selected_scene_sprite_index,
                        });
                    }
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(sprite_select_tool::SpriteSelectTool {
                        wh,
                        asset_docs: asset_docs.clone(),
                        select_part_option,
                    });
                }),
                table::fixed(320.px(), |wh, ctx| {
                    if let Some(position) =
                        selected_scene_sprite.map(|sprite| sprite.circumcircle.xy)
                    {
                        ctx.add(position_tool::PositionTool {
                            wh,
                            position,
                            on_change_position,
                        });
                    }
                }),
                table::fixed(320.px(), |wh, ctx| {
                    if let Some(size_radius) =
                        selected_scene_sprite.map(|sprite| sprite.circumcircle.radius)
                    {
                        ctx.add(size_tool::SizeTool {
                            wh,
                            size_radius,
                            on_change_size_radius,
                        });
                    }
                }),
            ])(wh, ctx)
        });
    }
}
