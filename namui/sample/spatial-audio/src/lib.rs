use namui::*;
use namui_prebuilt::simple_rect;

register_assets!();

static NEXT_SOURCE_ID_ATOM: Atom<usize> = Atom::uninitialized();

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(SpatialAudioExample {});
    })
}

struct SpatialAudioExample;

impl Component for SpatialAudioExample {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = system::screen::size();
        let screen_w = screen_wh.width.as_i32() as f32;
        let screen_h = screen_wh.height.as_i32() as f32;

        let (cam_x, set_cam_x) = ctx.state(|| 0.0f32);
        let (cam_y, set_cam_y) = ctx.state(|| 0.0f32);
        let (sources, set_sources) = ctx.state(|| Vec::<(usize, f32, f32)>::new());
        let (zoom, set_zoom) = ctx.state(|| 1.0f32);
        let (next_source_id, _) = ctx.init_atom(&NEXT_SOURCE_ID_ATOM, || 1usize);

        let speed = 3.0f32;
        ctx.on_raw_event(|event| {
            match event {
                RawEvent::ScreenRedraw => {
                    if system::keyboard::any_code_press([Code::ArrowLeft]) {
                        set_cam_x.mutate(move |x| *x -= speed);
                    }
                    if system::keyboard::any_code_press([Code::ArrowRight]) {
                        set_cam_x.mutate(move |x| *x += speed);
                    }
                    if system::keyboard::any_code_press([Code::ArrowUp]) {
                        set_cam_y.mutate(move |y| *y -= speed);
                    }
                    if system::keyboard::any_code_press([Code::ArrowDown]) {
                        set_cam_y.mutate(move |y| *y += speed);
                    }
                }
                _ => {}
            }
        });

        let cx = screen_w / 2.0;
        let cy = screen_h / 2.0;
        let z = *zoom;
        let next_id = *next_source_id;

        let listener_pos = (*cam_x, *cam_y, -200.0f32);
        let source_positions: Vec<_> = sources.iter().map(|&(id, x, y)| (id, x, y, 200.0f32)).collect();

        let mut debug_text = format!(
            "Listener: ({:.0}, {:.0}, {:.0})",
            listener_pos.0, listener_pos.1, listener_pos.2
        );
        for (id, x, y, sz) in &source_positions {
            debug_text += &format!("\nSrc {}: ({:.0}, {:.0}, {:.0})", id, x, y, sz);

            let dx = listener_pos.0 - x;
            let dy = listener_pos.1 - y;
            let dz = listener_pos.2 - sz;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            let azimuth_rad = dx.atan2(dz);
            debug_text += &format!("  dist={:.0} az={:.1}°", dist, azimuth_rad.to_degrees());
        }

        ctx.add(namui::text(TextParam {
            text: debug_text,
            x: 10.px(),
            y: 10.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font: Font {
                size: 14.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            style: TextStyle {
                color: Color::BLACK,
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.compose(|ctx| {
            let ctx = ctx
                .translate((px(cx), px(cy)))
                .scale(Xy::new(z, z))
                .translate((px(-*cam_x), px(-*cam_y)));

            ctx.compose(|ctx| {
                ctx.translate((px(*cam_x), px(*cam_y)))
                    .add(AudioGroup {
                        volume: 1.0,
                        z: -200.0,
                        children: |ctx: ComposeCtx| {
                            ctx.add(AudioListener);
                        },
                    })
                    .add(filled_circle(20.0, Color::BLUE));
            });

            for &(id, x, y) in sources.iter() {
                ctx.compose_with_key(id as u128, |ctx| {
                    ctx.translate((px(x), px(y)))
                        .add(AudioGroup {
                            volume: 1.0,
                            z: 200.0,
                            children: |ctx: ComposeCtx| {
                                ctx.add(Audio {
                                    asset: asset::SOUND,
                                    repeat: true,
                                    spatial: true,
                                });
                            },
                        })
                        .add(filled_circle(10.0, Color::RED));
                });
            }

            ctx.compose(|ctx| {
                ctx.add(simple_rect(
                    Wh::new(px(screen_w), px(screen_h)),
                    Color::grayscale_f01(0.0),
                    1.px(),
                    Color::grayscale_f01(0.94),
                ));

                ctx.attach_event(|event| match event {
                    Event::MouseUp { event } => {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        let local = event.local_xy();
                        let lx = local.x.as_f32();
                        let ly = local.y.as_f32();
                        match event.button {
                            Some(MouseButton::Left) => {
                                let id = next_id;
                                NEXT_SOURCE_ID_ATOM.set(next_id + 1);
                                set_sources.mutate(move |s| {
                                    s.push((id, lx, ly));
                                });
                            }
                            Some(MouseButton::Right) => {
                                set_sources.mutate(move |s| {
                                    if s.is_empty() {
                                        return;
                                    }
                                    let nearest = s
                                        .iter()
                                        .enumerate()
                                        .min_by(|(_, a), (_, b)| {
                                            let da =
                                                (a.1 - lx).powi(2) + (a.2 - ly).powi(2);
                                            let db =
                                                (b.1 - lx).powi(2) + (b.2 - ly).powi(2);
                                            da.partial_cmp(&db).unwrap()
                                        })
                                        .map(|(i, _)| i);
                                    if let Some(i) = nearest {
                                        s.remove(i);
                                    }
                                });
                            }
                            _ => {}
                        }
                    }
                    Event::Wheel { event } => {
                        let delta = event.delta_xy.y;
                        set_zoom.mutate(move |z| {
                            *z = (*z * (1.0 - delta * 0.001)).clamp(0.1, 10.0);
                        });
                    }
                    _ => {}
                });
            });
        });
    }
}

fn filled_circle(radius: f32, color: Color) -> RenderingTree {
    let oval_rect = Rect::Xywh {
        x: px(-radius),
        y: px(-radius),
        width: px(radius * 2.0),
        height: px(radius * 2.0),
    };
    let path = Path::new().add_oval(oval_rect);
    let paint = Paint::new(color)
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);
    namui::path(path, paint)
}
