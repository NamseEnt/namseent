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

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(210.px(), 350.px())).add(customer);
        });
        /*
            TODO: 재고 박스에서 클릭해서 아이템 얻고, 손님 눌러서 제공해.
        */

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));
    }
}

fn customer(ctx: &RenderCtx) {
    let head_radius = 40.px();
    let neck_length = 20.px();
    let body_length = 100.px();
    let arm_length = 60.px();
    let leg_length = 60.px();

    /*
         O  <-- Bubble here
         |
        /|\
         |
        / \
    */

    // --- Person Drawing ---
    let person_path = Path::new()
        // Head
        .add_oval(Rect::from_xy_wh(
            Xy::zero(),
            Wh::new(head_radius * 2, head_radius * 2),
        ))
        // Neck + Body
        .move_to(head_radius, head_radius * 2) // Top of neck
        .line_to(head_radius, head_radius * 2 + neck_length + body_length) // Bottom of body (Leg joint)
        // Arms
        .move_to(head_radius, head_radius * 2 + neck_length) // Arm joint (Bottom of neck)
        .line_to(
            head_radius - arm_length,      // x coordinate for left hand (horizontal)
            head_radius * 2 + neck_length, // y coordinate for left hand (same as joint)
        ) // Left hand (90 deg)
        .move_to(head_radius, head_radius * 2 + neck_length) // Arm joint
        .line_to(
            head_radius + arm_length,      // x coordinate for right hand (horizontal)
            head_radius * 2 + neck_length, // y coordinate for right hand (same as joint)
        ) // Right hand (90 deg)
        // Legs
        .move_to(head_radius, head_radius * 2 + neck_length + body_length) // Leg joint
        .line_to(
            head_radius - leg_length,
            head_radius * 2 + neck_length + body_length + leg_length,
        ) // Left foot (approx 45 deg)
        .move_to(head_radius, head_radius * 2 + neck_length + body_length) // Leg joint
        .line_to(
            head_radius + leg_length,
            head_radius * 2 + neck_length + body_length + leg_length,
        ); // Right foot (approx 45 deg)

    let person_paint = Paint::new(Color::WHITE)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.px());

    ctx.add(namui::path(person_path, person_paint));

    ctx.add(namui::text(TextParam {
        text: "1번 2개 주세요".to_string(),
        x: head_radius,
        y: -30.px(),
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font: Font {
            name: "NotoSansKR-Regular".to_string(),
            size: 24.int_px(),
        },
        style: TextStyle {
            color: Color::WHITE,
            ..Default::default()
        },
        max_width: None,
    }));
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
            ctx.compose(|ctx| {
                ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                    .add(namui::translate(x_offset, y_offset, hole.clone()));
            });
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

    ctx.add(border);

    let keyring_path = {
        let rect_wh = Wh::new(12.px(), 16.px());
        let ring_radius = 4.px();
        let chain_length = 8.px();

        Path::new()
            .add_oval(Rect::from_xy_wh(
                Xy::new(rect_wh.width / 2 - ring_radius, 0.px()),
                Wh::new(ring_radius * 2, ring_radius * 2),
            ))
            .move_to(rect_wh.width / 2, ring_radius * 2)
            .line_to(rect_wh.width / 2, ring_radius * 2 + chain_length)
            .add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), ring_radius * 2 + chain_length),
                rect_wh,
            ))
    };
    let keyring_front_paint = Paint::new(Color::WHITE)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.px());
    let keyring_back_paint = Paint::new(Color::BLACK)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.px());

    for x in 0..3 {
        for y in 0..2 {
            let x_offset = (hole_wh.width * width_hole) / 3 * (x as f32 + 0.5);
            let y_offset = (hole_wh.height * height_hole) / 2 * (y as f32 + 0.5);
            ctx.add(namui::translate(
                x_offset,
                y_offset,
                render([
                    namui::path(keyring_path.clone(), keyring_front_paint.clone()),
                    namui::path(keyring_path.clone(), keyring_back_paint.clone()),
                ]),
            ));
        }
    }

    let hole = simple_rect(
        hole_wh,
        Color::grayscale_f01(0.5),
        1.px(),
        Color::TRANSPARENT,
    );

    for x in 0..width_hole {
        for y in 0..height_hole {
            let x_offset = hole_wh.width * x;
            let y_offset = hole_wh.height * y;
            ctx.add(namui::translate(x_offset, y_offset, hole.clone()));
        }
    }
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
