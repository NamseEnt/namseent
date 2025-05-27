use super::PreviewKind;
use crate::{
    MapCoordF32,
    game_state::{
        TILE_PX_SIZE, TRAVEL_POINTS,
        item::{Item, ItemUsage, linear_area_rect_points, use_item},
        mutate_game_state,
    },
};
use namui::*;

pub struct ItemCursorPreview<'a> {
    pub item: &'a Item,
    pub item_index: usize,
    pub map_coord: MapCoordF32,
}
impl Component for ItemCursorPreview<'_> {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self {
            item,
            item_index,
            map_coord,
        } = self;

        let cancel_using_item = || {
            mutate_game_state(|game_state| {
                game_state.cursor_preview.kind = PreviewKind::None;
            });
        };

        let use_item = || {
            let item = item.clone();
            mutate_game_state(move |game_state| {
                use_item(game_state, &item, Some(map_coord));
                game_state.items.remove(item_index);
                game_state.cursor_preview.kind = PreviewKind::None;
            });
        };

        let ctx = ctx.translate(TILE_PX_SIZE.to_xy() * map_coord);

        match item.kind.usage() {
            ItemUsage::Instant => {}
            ItemUsage::CircularArea { radius } => {
                let radius = TILE_PX_SIZE * radius;
                let path =
                    Path::new().add_oval(Rect::from_xy_wh(radius.to_xy() * -1.0, radius * 2.0));
                let fill_paint = Paint::new(Color::RED.with_alpha(128));
                let stroke_paint =
                    Paint::new(Color::RED.brighter(0.5)).set_style(PaintStyle::Stroke);
                ctx.add(namui::path(path.clone(), stroke_paint));
                ctx.add(namui::path(path, fill_paint));
            }
            ItemUsage::LinearArea { thickness } => {
                let points = linear_area_rect_points(
                    TRAVEL_POINTS.last().unwrap().map(|u| u as f32 + 0.5),
                    map_coord,
                    thickness,
                )
                .map(|tile| TILE_PX_SIZE.to_xy() * (tile - map_coord));

                let path = Path::new().add_poly(&points, true);
                let fill_paint = Paint::new(Color::RED.with_alpha(128));
                let stroke_paint =
                    Paint::new(Color::RED.brighter(0.5)).set_style(PaintStyle::Stroke);

                ctx.add(namui::path(path.clone(), stroke_paint));
                ctx.add(namui::path(path, fill_paint));
            }
        }

        ctx.attach_event(|event| match event {
            Event::MouseDown { event } => match event.button {
                Some(MouseButton::Left) => {
                    use_item();
                    event.stop_propagation();
                }
                Some(MouseButton::Right) => {
                    cancel_using_item();
                    event.stop_propagation();
                }
                _ => {}
            },
            Event::KeyDown { event } => {
                if event.code == Code::Escape {
                    cancel_using_item();
                    event.stop_propagation();
                }
            }
            _ => {}
        });
    }
}
