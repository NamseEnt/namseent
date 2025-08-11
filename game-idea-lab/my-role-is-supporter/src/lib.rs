use namui::*;
use namui_prebuilt::simple_rect;

pub fn main() {
    namui::start(game);
}

fn game(ctx: &RenderCtx) {
    let (beat_conveyor_belt, set_beat_conveyor_belt) = ctx.state(BeatConveyorBelt::new);
    let (fast_beat_conveyor_belt, set_fast_beat_conveyor_belt) =
        ctx.state(FastBeatConveyorBelt::new);

    ctx.interval("beat_conveyor_belt", (1000. / 60.).ms(), move |dt| {
        set_beat_conveyor_belt.mutate(move |beat_conveyor_belt| {
            beat_conveyor_belt.tick(dt);
        });
        set_fast_beat_conveyor_belt.mutate(move |fast_beat_conveyor_belt| {
            fast_beat_conveyor_belt.tick(dt);
        });
    });

    let goal_x = 500.px();
    let goal_y = 500.px();

    ctx.translate(Xy::new(goal_x, goal_y - 50.px()))
        .add(simple_rect(
            Wh::new(1.px(), 100.px()),
            Color::RED,
            1.px(),
            Color::RED,
        ));

    let note_rt = simple_rect(
        Wh::new(3.px(), 16.px()),
        Color::RED,
        2.px(),
        Color::from_f01(1., 0.5, 0., 1.),
    );

    for beat in beat_conveyor_belt.visible_beats() {
        let x = goal_x - waveform(beat, 50.px());
        ctx.translate(Xy::new(x.floor(), goal_y))
            .add(note_rt.clone());
    }

    for beat in fast_beat_conveyor_belt.visible_beats() {
        let x = goal_x - waveform(beat, 200.px());
        ctx.translate(Xy::new(x.floor(), goal_y - 20.px()))
            .add(note_rt.clone());
    }

    ctx.add(simple_rect(
        Wh::new(1000.px(), 1000.px()),
        Color::TRANSPARENT,
        0.px(),
        Color::BLACK,
    ));
}

fn waveform(x: f32, velocity: Px) -> Px {
    let interval = 1.;

    let r = x % interval;
    let q = x / interval - r;

    let eased = ease_in_out_quint(r);

    velocity * (q + eased)
}
fn ease_in_out_quint(x: f32) -> f32 {
    if x < 0.5 {
        16. * x.powi(5)
    } else {
        1. - (-2. * x + 2.).powi(5) / 2.
    }
}

const BARS: [&str; 2] = ["o---o---", "o-o-o-o-"];
const DURATION_PER_BAR: Duration = Duration::from_secs(3);

struct BeatConveyorBelt {
    first_bar: &'static str,
    second_bar: &'static str,
    t: Duration,
    time_per_beat: Duration,
}
const BEATS_PER_BAR: usize = 8;

impl BeatConveyorBelt {
    fn new() -> Self {
        Self {
            first_bar: BARS[0],
            second_bar: BARS[1],
            t: Duration::ZERO,
            time_per_beat: DURATION_PER_BAR / BEATS_PER_BAR as i32,
        }
    }
    fn tick(&mut self, dt: Duration) {
        let mut total_t = self.t + dt;
        let time_per_bar = self.time_per_beat * BEATS_PER_BAR as i32;
        while total_t >= time_per_bar {
            self.first_bar = self.second_bar;
            self.second_bar = BARS[namui::rand::random::<usize>() % BARS.len()];
            total_t -= time_per_bar;
        }
        self.t = total_t;
    }
    fn visible_beats(&self) -> Vec<f32> {
        let first_bar_index = self.t / self.time_per_beat;

        let mut tack = -first_bar_index;

        let mut beats = vec![];
        for char in self.first_bar.chars().chain(self.second_bar.chars()) {
            if char != '-' && tack > 0. {
                beats.push(tack);
            }
            tack += 1.;
        }

        beats
    }
}

const FAST_BEAT_BARS: [&str; 3] = ["--------", "----O-O-", "----O---"];
const FAST_DURATION_PER_BAR: Duration = Duration::from_secs(3);

struct FastBeatConveyorBelt {
    first_bar: &'static str,
    second_bar: &'static str,
    t: Duration,
    time_per_beat: Duration,
}
const FAST_BEATS_PER_BAR: usize = 8;

impl FastBeatConveyorBelt {
    fn new() -> Self {
        Self {
            first_bar: FAST_BEAT_BARS[0],
            second_bar: FAST_BEAT_BARS[1],
            t: Duration::ZERO,
            time_per_beat: FAST_DURATION_PER_BAR / FAST_BEATS_PER_BAR as i32,
        }
    }
    fn tick(&mut self, dt: Duration) {
        let mut total_t = self.t + dt;
        let time_per_bar = self.time_per_beat * FAST_BEATS_PER_BAR as i32;
        while total_t >= time_per_bar {
            self.first_bar = self.second_bar;
            self.second_bar = FAST_BEAT_BARS[namui::rand::random::<usize>() % FAST_BEAT_BARS.len()];
            total_t -= time_per_bar;
        }
        self.t = total_t;
    }
    fn visible_beats(&self) -> Vec<f32> {
        let first_bar_index = self.t / self.time_per_beat;

        let mut tack = -first_bar_index;

        let mut beats = vec![];
        for char in self.first_bar.chars().chain(self.second_bar.chars()) {
            if char != '-' && tack > 0. {
                beats.push(tack);
            }
            tack += 1.;
        }

        beats
    }
}
