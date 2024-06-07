use crate::app::theme::THEME;
use keyframe::{ease, functions::EaseOutQuint};
use namui::{time::since_start, *};
use namui_prebuilt::typography;

static RECENT_JUDGE: Atom<RecentJudge> = Atom::uninitialized();

#[component]
pub struct JudgeIndicator {
    pub wh: Wh<Px>,
}
impl Component for JudgeIndicator {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let (recent_judge, _set_recent_judge) = ctx.init_atom(&RECENT_JUDGE, RecentJudge::new);

        ctx.compose(|ctx| {
            let Some((alpha, scale)) =
                calculate_alpha_and_scale(since_start() - recent_judge.judge_at)
            else {
                return;
            };
            ctx.translate((wh.width / 3, wh.height / 2))
                .scale(scale)
                .add(typography::effect::glow(
                    recent_judge.to_string(),
                    Font {
                        size: typography::adjust_font_size(wh.height / 2.0),
                        name: THEME.font_name.to_string(),
                    },
                    Xy::zero(),
                    Paint::new(Color::from_u8(255, 255, 255, alpha)),
                    TextAlign::Center,
                    TextBaseline::Middle,
                    BlurStyle::Normal,
                    blur_sigma::from_radius(4.0),
                    8.px(),
                    recent_judge.glow_color(alpha),
                ));
        });
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
impl std::fmt::Display for RecentJudge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.judge {
            Judge::Perfect { combo } => write!(f, "Perfect\n{}", combo),
            Judge::Good { combo } => write!(f, "Good\n{}", combo),
            Judge::Miss => write!(f, "Miss"),
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
    let animation_duration = 2.sec();
    if duration > animation_duration {
        return None;
    }
    let progress = (duration / animation_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return None;
    }
    let time_function = ease(EaseOutQuint, 0.0, 1.0, progress);
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
