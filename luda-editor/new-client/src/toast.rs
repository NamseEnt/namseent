use super::*;

#[allow(dead_code)]
pub fn neutral(text: impl ToString) {
    toast(Variant::Neutral, text);
}
#[allow(dead_code)]
pub fn informative(text: impl ToString) {
    toast(Variant::Informative, text);
}
#[allow(dead_code)]
pub fn positive(text: impl ToString) {
    toast(Variant::Positive, text);
}
#[allow(dead_code)]
pub fn negative(text: impl ToString) {
    toast(Variant::Negative, text);
}

pub struct Toast;

static TOAST_MESSAGES_ATOM: Atom<Vec<ToastMessage>> = Atom::uninitialized();
const DISMISS_AFTER: Duration = Duration::from_secs(5);

impl Component for Toast {
    fn render(self, ctx: &RenderCtx) {
        let (messages, set_messages) = ctx.init_atom(&TOAST_MESSAGES_ATOM, Vec::new);

        if messages
            .iter()
            .any(|x| x.added_at + DISMISS_AFTER < Instant::now())
        {
            set_messages.mutate(|messages| {
                messages.retain(|message| message.added_at + DISMISS_AFTER > Instant::now());
            });
        }

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        ctx.compose(|ctx| {
            const HEIGHT: Px = px(16.0 + 16.0 + 16.0 + 16.0);

            for (i, message) in messages.as_ref().iter().enumerate() {
                let y = screen_wh.height - 16.px() - HEIGHT / 2 - HEIGHT * i;

                let background_color = match message.variant {
                    Variant::Neutral => Color::grayscale_f01(0.7),
                    Variant::Informative => Color::BLUE,
                    Variant::Positive => Color::GREEN,
                    Variant::Negative => Color::RED,
                };

                ctx.add(text(TextParam {
                    text: message.text.clone(),
                    x: screen_wh.width / 2,
                    y,
                    align: TextAlign::Center,
                    baseline: TextBaseline::Middle,
                    font: Font {
                        size: 16.int_px(),
                        name: "NotoSansKR-Regular".to_string(),
                    },
                    style: TextStyle {
                        color: Color::WHITE,
                        background: Some(TextStyleBackground {
                            color: background_color,
                            margin: Some(Ltrb {
                                left: 16.px(),
                                top: 16.px(),
                                right: 16.px(),
                                bottom: 16.px(),
                            }),
                            round: Some(4.px()),
                        }),
                        ..Default::default()
                    },
                    max_width: Some(screen_wh.width / 2),
                }));
            }
        });
    }
}

struct ToastMessage {
    variant: Variant,
    text: String,
    added_at: Instant,
}

enum Variant {
    Neutral,
    Informative,
    Positive,
    Negative,
}

fn toast(variant: Variant, text: impl ToString) {
    let text = text.to_string();
    TOAST_MESSAGES_ATOM.mutate(move |messages| {
        messages.push(ToastMessage {
            variant,
            text,
            added_at: Instant::now(),
        });
    });
}
