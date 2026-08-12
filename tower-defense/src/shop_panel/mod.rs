mod constants;
mod items;
mod next_fab;
mod paper_content;
mod slot_layout_calculator;
mod slot_renderer;
mod slot_rendering_data;
mod voyager;

use crate::animation::with_spring;
use crate::game_state::use_game_state;
use crate::hand::xy_with_spring;
use crate::shop_panel::next_fab::ShopNextFab;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};

use constants::{BG_HEIGHT, OUTSIDE_HEIGHT, SHOP_PANEL_HEIGHT, shop_panel_wh};
use namui::*;
use namui_prebuilt::simple_rect;

use paper_content::ShopPaperContent;
// use voyager::Voyager;

pub struct ShopPanel;

struct ShopPanelLayout {
    pub panel_wh: Wh<Px>,
    pub bg_y: Px,
    pub closed_xy: Xy<Px>,
    pub target_xy: Xy<Px>,
}

impl ShopPanelLayout {
    #[inline]
    fn compute(panel_open: bool, screen_wh: Wh<Px>) -> Self {
        let panel_wh = shop_panel_wh();
        let bg_y = SHOP_PANEL_HEIGHT - BG_HEIGHT;
        let center_x = (screen_wh.width - panel_wh.width) / 2.0;
        let closed_xy = Xy::new(center_x, screen_wh.height + OUTSIDE_HEIGHT);
        let open_y = screen_wh.height + OUTSIDE_HEIGHT - bg_y - BG_HEIGHT;
        let open_xy = Xy::new(center_x, open_y);
        let target_xy = if panel_open { open_xy } else { closed_xy };

        ShopPanelLayout {
            panel_wh,
            bg_y,
            closed_xy,
            target_xy,
        }
    }
}

impl Component for ShopPanel {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let panel_open = game_state.can_open_shop_panel();
        let layout = ShopPanelLayout::compute(panel_open, screen_wh);
        let animated_xy = xy_with_spring(ctx, layout.target_xy, layout.closed_xy);
        let backdrop_progress = with_spring(
            ctx,
            if panel_open { 1.0 } else { 0.0 },
            0.0,
            |value| value * value,
            || 0.0,
        );
        let backdrop_alpha = (backdrop_progress * 180.0).clamp(0.0, 180.0) as u8;

        ctx.add(ShopNextFab {
            screen_wh,
            visible: panel_open,
        });

        ctx.absolute(animated_xy).compose(|ctx| {
            ctx.translate((0.px(), layout.bg_y)).add(ShopPaperContent {
                wh: Wh::new(layout.panel_wh.width, SHOP_PANEL_HEIGHT),
            });

            ctx.translate((0.px(), layout.bg_y))
                .add(PaperContainerBackground {
                    width: layout.panel_wh.width,
                    height: BG_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Paper,
                    color: crate::theme::palette::SURFACE_CONTAINER_LOWEST,
                    outline_color: None,
                    shadow: true,
                    arrow: None,
                });

            // ctx.add(Voyager);
        });

        if backdrop_alpha > 0 {
            ctx.add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::BLACK.with_alpha(backdrop_alpha),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseUp { event }
                    | Event::MouseMove { event } => event.stop_propagation(),
                    Event::Wheel { event } => event.stop_propagation(),
                    _ => {}
                }),
            );
        }
    }
}
