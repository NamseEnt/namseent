use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(App {});
    });
}

struct App {}
impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(20.px(), 404.px()))
                .add(goods_stock_box);
        });

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(330.px(), 260.px()))
                .add(display_stand);
        });

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(10.px(), 500.px())).add(booth_table);
        });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));
    }
}

fn goods_stock_box(ctx: &RenderCtx) {
    let hole_wh = Wh::new(48.px(), 24.px());
    let width_hole = 3;
    let height_hole = 4;

    let hole = simple_rect(hole_wh, Color::WHITE, 1.px(), Color::grayscale_f01(0.8));

    for x in 0..width_hole {
        for y in 0..height_hole {
            let x_offset = hole_wh.width * x;
            let y_offset = hole_wh.height * y;
            ctx.add(namui::translate(x_offset, y_offset, hole.clone()));
        }
    }
}

fn display_stand(ctx: &RenderCtx) {
    let hole_wh = Wh::new(16.px(), 16.px());
    let width_hole = 8;
    let height_hole = 15;

    let border = simple_rect(
        Wh::new(hole_wh.width * width_hole, hole_wh.height * height_hole),
        Color::WHITE,
        4.px(),
        Color::TRANSPARENT,
    );

    let hole = simple_rect(hole_wh, Color::WHITE, 1.px(), Color::TRANSPARENT);

    for x in 0..width_hole {
        for y in 0..height_hole {
            let x_offset = hole_wh.width * x;
            let y_offset = hole_wh.height * y;
            ctx.add(namui::translate(x_offset, y_offset, hole.clone()));
        }
    }

    ctx.add(border);
}

fn booth_table(ctx: &RenderCtx) {
    let dark_brown = Color::from_u8(0x8B, 0x45, 0x13, 0xFF);
    let light_brown = Color::from_u8(0xD2, 0xB4, 0x8C, 0xFF);
    let ivory = Color::from_u8(0xFF, 0xF8, 0xDC, 0xFF);

    // 상판
    let table_top = simple_rect(Wh::new(480.px(), 20.px()), Color::WHITE, 1.px(), dark_brown);
    // 다리
    let table_leg = simple_rect(
        Wh::new(20.px(), 100.px()),
        Color::WHITE,
        1.px(),
        light_brown,
    );
    // 가리개
    let table_cover = simple_rect(Wh::new(480.px(), 75.px()), Color::WHITE, 1.px(), ivory);

    ctx.add(table_top);
    ctx.add(namui::translate(20.px(), 20.px(), table_leg.clone()));
    ctx.add(namui::translate(440.px(), 20.px(), table_leg));
    ctx.add(table_cover);
}
