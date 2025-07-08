use crate::game_state::theme::palette;
use namui::*;

pub struct MonsterHpBar {
    pub wh: Wh<Px>,
    pub progress: f32,
}

impl Component for MonsterHpBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, progress } = self;
        let container_rect = Rect::from_xy_wh(wh.to_xy() * -0.5, wh);
        ctx.add(rect(RectParam {
            rect: Rect::from_xy_wh(container_rect.xy(), Wh::new(wh.width * progress, wh.height)),
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill { color: Color::RED }),
                round: None,
            },
        }));
        ctx.add(rect(RectParam {
            rect: container_rect,
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Outside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: None,
            },
        }));
    }
}
