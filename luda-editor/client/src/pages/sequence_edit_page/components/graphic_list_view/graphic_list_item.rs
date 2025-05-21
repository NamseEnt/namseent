use crate::{
    color,
    pages::sequence_edit_page::components::graphic_list_view::graphic_thumbnail::GraphicThumbnail,
};
use namui::*;
use namui_prebuilt::{simple_rect, table};
use rpc::data::ScreenGraphic;

pub struct GraphicListItem<'a> {
    pub project_id: Uuid,
    pub wh: Wh<Px>,
    pub graphic: &'a ScreenGraphic,
    pub is_selected: bool,
}
impl Component for GraphicListItem<'_> {
    fn render(self, ctx: &RenderCtx)  {
        const PADDING: Px = px(4.0);

        let Self {
            project_id,
            wh,
            graphic,
            is_selected,
        } = self;
        let graphic_name = match graphic {
            ScreenGraphic::Image(image) => image.id.to_string(),
            ScreenGraphic::Cg(cg) => cg.name.clone(),
        };

        let stroke_color = color::stroke_color(is_selected, false);
        let stroke_width = match is_selected {
            true => 2.px(),
            false => 1.px(),
        };

        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed(wh.height, |wh, ctx| {
                    ctx.add(GraphicThumbnail {
                        project_id,
                        wh,
                        graphic,
                    });
                }),
                table::ratio(1, |wh, ctx| {
                    table::padding(PADDING, |wh, ctx| {
                        ctx.add(namui_prebuilt::typography::body::left(
                            wh.height,
                            graphic_name,
                            stroke_color,
                        ));
                    })(wh, ctx);
                }),
            ])(wh, ctx);
        });

        ctx.component(simple_rect(
            wh,
            stroke_color,
            stroke_width,
            color::BACKGROUND,
        ));

        
    }
}
