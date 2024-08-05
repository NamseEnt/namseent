mod position_tool;
mod scene_sprite_list;
mod size_tool;
mod sprite_select_tool;

use namui::*;
use namui_prebuilt::*;

pub struct SceneSpriteEditor<'a> {
    pub wh: Wh<Px>,
}

impl Component for SceneSpriteEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(320.px(), {
                    |wh, ctx| {
                        ctx.add(scene_sprite_list::SceneSpriteList {
                            wh,
                            scene_sprites,
                            remove_scene_sprite,
                            add_new_scene_sprite,
                            move_scene_sprite_up_down,
                            select_scene_sprite,
                            selected_scene_sprite_index,
                        });
                    }
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(sprite_select_tool::SpriteSelectTool {
                        wh,
                        sprite_docs: todo!(),
                    });
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(position_tool::PositionTool {
                        wh,
                        position: todo!(),
                        on_change_position: todo!(),
                    });
                }),
                table::fixed(320.px(), |wh, ctx| {
                    ctx.add(size_tool::SizeTool { wh });
                }),
            ])(wh, ctx)
        });
    }
}
