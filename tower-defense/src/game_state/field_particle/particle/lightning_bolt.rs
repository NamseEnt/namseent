use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

// 번개줄기 점 생성 상수
const LIGHTNING_BOLT_MIN_POINTS: usize = 4;
const LIGHTNING_BOLT_MAX_POINTS: usize = 6;
const LIGHTNING_BOLT_OFFSET_RANGE: f32 = 0.2; // 점의 위치 변위 범위 (타일 단위)

// 번개줄기 spawn 상수
const LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS: i64 = 50; // 최소 수명
const LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS: i64 = 150; // 최대 수명
const LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION: f32 = 0.8; // 재귀 spawn 시 확률 감소 비율
const LIGHTNING_BOLT_END_OFFSET_RANGE: f32 = 0.2; // spawn 시 끝 지점 위치 변위 범위 (타일 단위)

// 렌더링 상수
const LIGHTNING_BOLT_LINE_THICKNESS_FULL: f32 = 0.01; // 전체 라인 두께
const LIGHTNING_BOLT_LINE_THICKNESS_MID: f32 = 0.03; // 중간 라인 두께
const LIGHTNING_BOLT_LINE_THICKNESS_CENTER: f32 = 0.05; // 중앙 라인 두께
const LIGHTNING_BOLT_LINE_INNER_THICKNESS_RATIO: f32 = 0.4; // 내부 라인 두께 비율
const LIGHTNING_BOLT_ALPHA_APPEAR_PHASE: f32 = 0.2; // alpha 나타나는 단계

// 푸른색 상수
const OUTER_COLOR_RGB: (f32, f32, f32) = (0.2, 0.5, 1.0); // 푸른 외곽
const INNER_COLOR_RGB: (f32, f32, f32) = (0.6, 0.85, 1.0); // 밝은 푸른 내부/흰색 기조
const INNER_COLOR_ALPHA_RATIO: f32 = 0.8; // 내부 라인의 alpha 비율

#[derive(Clone, State)]
pub struct LightningBoltParticle {
    pub points: Vec<(f32, f32)>, // 번개줄기의 점들 (타일 좌표)
    pub created_at: Instant,
    pub duration: Duration,
    pub alpha: f32,
    pub spawn_chance: f32, // 죽을 때 새로운 번개줄기를 생성할 확률 (0.0~1.0)
    pub has_spawned: bool, // 이미 spawn했는지 여부
}

impl LightningBoltParticle {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        spawn_chance: f32,
    ) -> Self {
        let points = Self::generate_points(start_xy, end_xy);

        Self {
            points,
            created_at,
            duration,
            alpha: 1.0,
            spawn_chance,
            has_spawned: false,
        }
    }

    /// 시작점과 끝점 사이의 경로를 따라 랜덤하게 배치된 점들을 생성
    fn generate_points(start_xy: (f32, f32), end_xy: (f32, f32)) -> Vec<(f32, f32)> {
        let mut rng = rand::thread_rng();
        let point_count = rng.gen_range(LIGHTNING_BOLT_MIN_POINTS..=LIGHTNING_BOLT_MAX_POINTS);

        // 방향 벡터
        let dx = end_xy.0 - start_xy.0;
        let dy = end_xy.1 - start_xy.1;
        let length = (dx * dx + dy * dy).sqrt();

        // 수직 벡터 (번개줄기의 좌우 방향)
        let perp_x = -dy / length.max(0.001);
        let perp_y = dx / length.max(0.001);

        let mut points = Vec::with_capacity(point_count);

        for i in 0..point_count {
            let t = i as f32 / (point_count - 1) as f32;

            // 기본 위치 (직선상의 점)
            let base_x = start_xy.0 + dx * t;
            let base_y = start_xy.1 + dy * t;

            // 좌우로 랜덤 오프셋 추가 (처음과 끝은 오프셋 없음)
            let offset = if i == 0 || i == point_count - 1 {
                0.0
            } else {
                rng.gen_range(-LIGHTNING_BOLT_OFFSET_RANGE..LIGHTNING_BOLT_OFFSET_RANGE)
            };

            let x = base_x + perp_x * offset;
            let y = base_y + perp_y * offset;

            points.push((x, y));
        }

        points
    }

    pub fn tick(
        &mut self,
        now: Instant,
        _dt: Duration,
    ) -> Option<crate::game_state::field_particle::FieldParticle> {
        self.alpha = self.current_alpha(now);

        // 죽기 직전에 spawn_chance 확률로 새로운 번개줄기 생성
        if !self.has_spawned && self.is_done(now) && self.points.len() >= 2 {
            self.has_spawned = true;
            if let Some(new_lightning) = self.try_spawn_child(now) {
                return Some(
                    crate::game_state::field_particle::FieldParticle::LightningBolt {
                        particle: new_lightning,
                    },
                );
            }
        }

        None
    }

    /// 일정 확률로 자식 번개줄기를 생성
    fn try_spawn_child(&self, now: Instant) -> Option<LightningBoltParticle> {
        let mut rng = rand::thread_rng();

        if rng.gen_range(0.0..1.0) >= self.spawn_chance {
            return None;
        }

        // 번개줄기의 중간 지점에서 새로운 번개줄기 시작
        let mid_idx = self.points.len() / 2;
        let start_xy = self.points[mid_idx];

        // 끝 지점은 마지막 점 근처의 랜덤 위치
        let last_xy = self.points[self.points.len() - 1];
        let offset_x =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let offset_y =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let end_xy = (last_xy.0 + offset_x, last_xy.1 + offset_y);

        // 자식 번개의 수명은 랜덤
        let duration =
            Duration::from_millis(rng.gen_range(
                LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS..LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS,
            ));

        // spawn_chance는 점점 낮아짐 (재귀적으로 spawn이 계속되지 않도록)
        let new_spawn_chance = self.spawn_chance * LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION;

        Some(LightningBoltParticle::new(
            start_xy,
            end_xy,
            now,
            duration,
            new_spawn_chance,
        ))
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 || self.points.len() < 2 {
            return RenderingTree::Empty;
        }

        // 점들을 연결하는 여러 개의 라인을 그려서 중앙을 두껍게 표현
        let mut renders = Vec::new();

        // 기본 전체 라인
        let line_configs = [(0, self.points.len(), LIGHTNING_BOLT_LINE_THICKNESS_FULL)];
        self.render_line_with_configs(&line_configs, &mut renders);

        // 중간 라인 (양 끝 1개씩 제외)
        if self.points.len() > 2 {
            let line_configs = [(1, self.points.len() - 1, LIGHTNING_BOLT_LINE_THICKNESS_MID)];
            self.render_line_with_configs(&line_configs, &mut renders);
        }

        // 중앙 라인 (양 끝 2개씩 제외) - 가장 두껍게
        if self.points.len() > 4 {
            let line_configs = [(
                2,
                self.points.len() - 2,
                LIGHTNING_BOLT_LINE_THICKNESS_CENTER,
            )];
            self.render_line_with_configs(&line_configs, &mut renders);
        }

        namui::render(renders)
    }

    /// 주어진 설정에 따라 라인을 렌더링
    fn render_line_with_configs(
        &self,
        configs: &[(usize, usize, f32)],
        renders: &mut Vec<RenderingTree>,
    ) {
        for &(start_idx, end_idx, thickness) in configs {
            // 외곽 라인 (푸른색)
            let outer_color = Color::from_f01(
                OUTER_COLOR_RGB.0,
                OUTER_COLOR_RGB.1,
                OUTER_COLOR_RGB.2,
                self.alpha,
            );

            let mut outer_path = Path::new();
            let first_point = self.points[start_idx];
            let first_px = TILE_PX_SIZE.to_xy() * Xy::new(first_point.0, first_point.1);
            outer_path = outer_path.move_to(first_px.x, first_px.y);

            for i in (start_idx + 1)..end_idx {
                let point = self.points[i];
                let point_px = TILE_PX_SIZE.to_xy() * Xy::new(point.0, point.1);
                outer_path = outer_path.line_to(point_px.x, point_px.y);
            }

            let outer_paint = Paint::new(outer_color)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(TILE_PX_SIZE.width * thickness)
                .set_stroke_cap(StrokeCap::Round)
                .set_stroke_join(StrokeJoin::Round)
                .set_blend_mode(BlendMode::Screen);

            renders.push(namui::path(outer_path, outer_paint));

            // 내부 라인 (더 밝은 색, 더 얇게)
            let inner_alpha = self.alpha * INNER_COLOR_ALPHA_RATIO;
            let inner_color = Color::from_f01(
                INNER_COLOR_RGB.0,
                INNER_COLOR_RGB.1,
                INNER_COLOR_RGB.2,
                inner_alpha,
            );

            let mut inner_path = Path::new();
            inner_path = inner_path.move_to(first_px.x, first_px.y);

            for i in (start_idx + 1)..end_idx {
                let point = self.points[i];
                let point_px = TILE_PX_SIZE.to_xy() * Xy::new(point.0, point.1);
                inner_path = inner_path.line_to(point_px.x, point_px.y);
            }

            let inner_paint = Paint::new(inner_color)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(
                    TILE_PX_SIZE.width * thickness * LIGHTNING_BOLT_LINE_INNER_THICKNESS_RATIO,
                )
                .set_stroke_cap(StrokeCap::Round)
                .set_stroke_join(StrokeJoin::Round)
                .set_blend_mode(BlendMode::Screen);

            renders.push(namui::path(inner_path, inner_paint));
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= self.duration {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        // Ease out: 처음에 빠르게 나타나고, 나중에 천천히 사라짐
        if progress < LIGHTNING_BOLT_ALPHA_APPEAR_PHASE {
            let appear_progress = progress / LIGHTNING_BOLT_ALPHA_APPEAR_PHASE;
            let inv = 1.0 - appear_progress;
            1.0 - (inv * inv)
        } else {
            let fade_progress = (progress - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE)
                / (1.0 - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE);
            1.0 - fade_progress
        }
    }
}
