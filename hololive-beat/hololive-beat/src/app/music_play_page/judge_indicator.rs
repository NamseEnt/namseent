use crate::app::color::THEME;
use namui::{math::num::traits::Pow, prelude::*, time::since_start};
use namui_prebuilt::typography;

static RECENT_JUDGE: Atom<RecentJudge> = Atom::uninitialized_new();

#[component]
pub struct JudgeIndicator {
    pub wh: Wh<Px>,
}
impl Component for JudgeIndicator {
    fn render(self, ctx: &namui::prelude::RenderCtx) -> namui::prelude::RenderDone {
        let Self { wh } = self;

        let (recent_judge, _set_recent_judge) = ctx.atom_init(&RECENT_JUDGE, RecentJudge::new);

        ctx.compose(|ctx| {
            let Some((alpha, scale)) =
                calculate_alpha_and_scale(since_start() - recent_judge.judge_at)
            else {
                return;
            };
            ctx.translate(wh.as_xy() / 2.0)
                .scale(scale)
                .add(typography::effect::glow(
                    recent_judge.to_string(),
                    Font {
                        size: typography::adjust_font_size(wh.height / 2.0),
                        name: "Fontspring-Demo-hemi_head_rg".to_string(),
                    },
                    Xy::zero(),
                    Paint::new(Color::from_u8(255, 255, 255, alpha)),
                    TextAlign::Center,
                    TextBaseline::Middle,
                    Blur::Normal {
                        sigma: Blur::convert_radius_to_sigma(4.0),
                    },
                    8.px(),
                    recent_judge.glow_color(alpha),
                ));
        });

        ctx.done()
    }
}

#[derive(Debug)]
struct RecentJudge {
    judge_at: Duration,
    judge: Judge,
}
impl RecentJudge {
    pub fn new() -> Self {
        Self {
            judge_at: Duration::from_secs(-100),
            judge: Judge::Miss,
        }
    }
    fn glow_color(&self, alpha: u8) -> Color {
        match self.judge {
            Judge::Perfect { .. } => {
                Color::from_u8(THEME.yellow.r, THEME.yellow.g, THEME.yellow.b, alpha)
            }
            Judge::Good { .. } => Color::from_u8(THEME.red.r, THEME.red.g, THEME.red.b, alpha),
            Judge::Miss => Color::from_u8(THEME.blue.r, THEME.blue.g, THEME.blue.b, alpha),
        }
    }
}
impl ToString for RecentJudge {
    fn to_string(&self) -> String {
        match self.judge {
            Judge::Perfect { combo } => format!("Perfect\n{combo}"),
            Judge::Good { combo } => format!("Good\n{combo}"),
            Judge::Miss => "Miss".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Judge {
    Perfect { combo: usize },
    Good { combo: usize },
    Miss,
}

fn calculate_alpha_and_scale(duration: Duration) -> Option<(u8, Xy<f32>)> {
    const T1: f32 = 0.9995;
    const T2: f32 = 0.99995;

    let animation_duration = 2.sec();
    if duration > animation_duration {
        return None;
    }
    let progress = (duration / animation_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return None;
    }
    let reverse_progress = 1.0 - progress;
    let time_function = T1 * (3.0_f32 * reverse_progress.pow(2) * progress)
        + T2 * (3.0 * reverse_progress * progress.pow(2))
        + progress.pow(3);
    let alpha = (255.0_f32 * (1.0_f32 - time_function)) as u8;
    let scale = Xy::single(0.75_f32 + (0.5_f32 * time_function));
    Some((alpha, scale))
}

pub fn indicate_judge(judge: Judge) {
    let recent_judge = RecentJudge {
        judge_at: since_start(),
        judge,
    };
    RECENT_JUDGE.set(recent_judge);
}
