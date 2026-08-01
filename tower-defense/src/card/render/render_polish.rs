use crate::rarity::Rarity;
use namui::*;

#[derive(Clone, Copy, PartialEq)]
enum SideSymbolKind {
    Diamond,
    Burr,
    Star,
}

const HALO_COMMON_MIN: f32 = 500.0;
const HALO_COMMON_MAX: f32 = 1000.0;
const HALO_RARE_MIN: f32 = 1500.0;
const HALO_RARE_MAX: f32 = 3000.0;
const HALO_EPIC_MIN: f32 = 3000.0;
const HALO_EPIC_MAX: f32 = 5000.0;
const HALO_LEGENDARY_MIN: f32 = 5000.0;
const HALO_LEGENDARY_MAX: f32 = 9000.0;

const HALO_COMMON_STRENGTH_MIN: f32 = 0.4;
const HALO_COMMON_STRENGTH_MAX: f32 = 0.6;
const HALO_RARE_STRENGTH_MIN: f32 = 0.8;
const HALO_RARE_STRENGTH_MAX: f32 = 1.0;
const HALO_EPIC_STRENGTH_MIN: f32 = 1.0;
const HALO_EPIC_STRENGTH_MAX: f32 = 1.2;
const HALO_LEGENDARY_STRENGTH_MIN: f32 = 1.0;
const HALO_LEGENDARY_STRENGTH_MAX: f32 = 1.2;

struct PolishCounts {
    bars: usize,
    chevrons: usize,
    symbol: Option<SideSymbolKind>,
    symbol_count: usize,
}

impl PolishCounts {
    fn empty() -> Self {
        Self {
            bars: 0,
            chevrons: 0,
            symbol: None,
            symbol_count: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.bars == 0 && self.chevrons == 0 && self.symbol.is_none()
    }

    fn from_percentage(percent: f32) -> Self {
        let percent = percent.clamp(0.0, 9000.0);
        if percent < 100.0 {
            return Self::empty();
        }

        let options = [
            (9000.0, 4, 3, Some(SideSymbolKind::Star), 5),
            (8000.0, 4, 3, Some(SideSymbolKind::Star), 4),
            (7000.0, 4, 3, Some(SideSymbolKind::Star), 3),
            (6000.0, 4, 3, Some(SideSymbolKind::Star), 2),
            (5000.0, 4, 3, Some(SideSymbolKind::Star), 1),
            (4000.0, 4, 3, Some(SideSymbolKind::Burr), 3),
            (3500.0, 4, 3, Some(SideSymbolKind::Burr), 2),
            (3000.0, 4, 3, Some(SideSymbolKind::Burr), 1),
            (2500.0, 4, 3, Some(SideSymbolKind::Diamond), 3),
            (2000.0, 4, 3, Some(SideSymbolKind::Diamond), 2),
            (1500.0, 4, 3, Some(SideSymbolKind::Diamond), 1),
            (1000.0, 4, 3, None, 0),
            (750.0, 4, 2, None, 0),
            (500.0, 4, 1, None, 0),
            (400.0, 4, 0, None, 0),
            (300.0, 3, 0, None, 0),
            (200.0, 2, 0, None, 0),
            (100.0, 1, 0, None, 0),
        ];

        for (threshold, bars, chevrons, symbol, symbol_count) in options {
            if percent >= threshold {
                return Self {
                    bars,
                    chevrons,
                    symbol,
                    symbol_count,
                };
            }
        }

        Self::empty()
    }
}

pub(super) fn render_polish_overlay(ctx: &RenderCtx, wh: Wh<Px>, bonus_pct: f32, opacity: f32) {
    if bonus_pct <= 0.0 {
        return;
    }

    let percent = bonus_pct * 100.0;
    let counts = PolishCounts::from_percentage(percent);
    if counts.is_empty() {
        return;
    }

    let alpha = (96.0 * opacity).round() as u8;
    let bar_chevron_color = Color::from_u8(0, 255, 0, alpha);
    let bar_chevron_paint = Paint::new(bar_chevron_color)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(wh.height * 0.075)
        .set_stroke_cap(StrokeCap::Round);
    let purple_color = Color::from_u8(160, 0, 255, alpha);
    let purple_fill_paint = Paint::new(purple_color).set_style(PaintStyle::Fill);
    let purple_stroke_paint = Paint::new(purple_color)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.px())
        .set_stroke_cap(StrokeCap::Round);

    let clip_ctx = ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect);
    if let Some(symbol) = counts.symbol {
        render_center_symbols(
            ctx,
            wh,
            symbol,
            counts.symbol_count,
            purple_fill_paint,
            purple_stroke_paint,
        );
    }
    render_center_markers(
        &clip_ctx,
        wh,
        counts.bars,
        counts.chevrons,
        bar_chevron_paint.clone(),
    );
}

pub fn polish_halo_config(bonus_pct: f32) -> Option<(Color, f32)> {
    if bonus_pct <= 0.0 {
        return None;
    }

    let percent = bonus_pct * 100.0;
    let counts = PolishCounts::from_percentage(percent);
    if counts.is_empty() {
        return None;
    }

    let progress = |min: f32, max: f32| ((percent - min) / (max - min)).clamp(0.0, 1.0);

    if counts.symbol.is_none() {
        if counts.chevrons == 0 {
            return None;
        }
        let strength = HALO_COMMON_STRENGTH_MIN
            + (HALO_COMMON_STRENGTH_MAX - HALO_COMMON_STRENGTH_MIN)
                * progress(HALO_COMMON_MIN, HALO_COMMON_MAX);
        return Some((Rarity::Common.color(), strength));
    }

    match counts.symbol.unwrap() {
        SideSymbolKind::Diamond => {
            let strength = HALO_RARE_STRENGTH_MIN
                + (HALO_RARE_STRENGTH_MAX - HALO_RARE_STRENGTH_MIN)
                    * progress(HALO_RARE_MIN, HALO_RARE_MAX);
            Some((Rarity::Rare.color(), strength))
        }
        SideSymbolKind::Burr => {
            let strength = HALO_EPIC_STRENGTH_MIN
                + (HALO_EPIC_STRENGTH_MAX - HALO_EPIC_STRENGTH_MIN)
                    * progress(HALO_EPIC_MIN, HALO_EPIC_MAX);
            Some((Rarity::Epic.color(), strength))
        }
        SideSymbolKind::Star => {
            let strength = HALO_LEGENDARY_STRENGTH_MIN
                + (HALO_LEGENDARY_STRENGTH_MAX - HALO_LEGENDARY_STRENGTH_MIN)
                    * progress(HALO_LEGENDARY_MIN, HALO_LEGENDARY_MAX);
            Some((Rarity::Legendary.color(), strength))
        }
    }
}

fn render_center_markers(ctx: &ComposeCtx, wh: Wh<Px>, bars: usize, chevrons: usize, paint: Paint) {
    let bar_height = wh.height * 0.09;
    let chevron_height = wh.height * 0.1;
    let full_width = wh.width;
    let line_x = 0.px();
    let chevron_margin = wh.height * 0.05;
    let mut chevron_y = wh.height * 0.5 - chevron_margin - chevron_height;

    for _ in 0..chevrons {
        let path = damage_chevron_path(Wh::new(full_width, chevron_height));
        ctx.translate(Xy::new(line_x, chevron_y))
            .add(namui::path(path.clone(), paint.clone()));
        chevron_y -= chevron_height;
    }

    let mut bar_y = wh.height * 0.5;
    for _ in 0..bars {
        let path = damage_bar_path(Wh::new(full_width, bar_height));
        ctx.translate(Xy::new(line_x, bar_y))
            .add(namui::path(path.clone(), paint.clone()));
        bar_y += bar_height + 2.px();
    }
}

fn render_center_symbols(
    ctx: &RenderCtx,
    wh: Wh<Px>,
    symbol: SideSymbolKind,
    count: usize,
    fill_paint: Paint,
    _stroke_paint: Paint,
) {
    if count == 0 {
        return;
    }

    let symbol_size = wh.width * 0.3;
    let center_y = wh.height * 0.5;
    let gap = symbol_size * -0.3;
    let total_width = symbol_size * count as f32 + gap * (count as f32 - 1.0).max(0.0);
    let start_x = (wh.width - total_width) * 0.5;

    if symbol == SideSymbolKind::Star && count == 5 {
        let center = Xy::new(wh.width * 0.5, center_y);
        let radius = symbol_size * 1.1;
        let star_path = damage_star_path(Wh::new(symbol_size, symbol_size));

        for i in 0..5 {
            let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * 2.0 * std::f32::consts::PI / 5.0;
            let x = center.x + radius * angle.cos() - symbol_size * 0.5;
            let y = center.y + radius * angle.sin() - symbol_size * 0.5;
            ctx.translate(Xy::new(x, y))
                .add(namui::path(star_path.clone(), fill_paint.clone()));
        }
        return;
    }

    for index in 0..count {
        let x = start_x + (symbol_size + gap) * index as f32;
        let y = center_y - symbol_size * 0.5;
        let symbol_path = match symbol {
            SideSymbolKind::Diamond => damage_diamond_path(Wh::new(symbol_size, symbol_size)),
            SideSymbolKind::Burr => damage_asterisk_path(Wh::new(symbol_size, symbol_size)),
            SideSymbolKind::Star => damage_star_path(Wh::new(symbol_size, symbol_size)),
        };
        let paint = match symbol {
            SideSymbolKind::Burr => fill_paint.clone(),
            SideSymbolKind::Diamond | SideSymbolKind::Star => fill_paint.clone(),
        };
        ctx.translate(Xy::new(x, y))
            .add(namui::path(symbol_path, paint));
    }
}

fn damage_bar_path(wh: Wh<Px>) -> Path {
    Path::new()
        .move_to(0.px(), wh.height * 0.5)
        .line_to(wh.width, wh.height * 0.5)
}

fn damage_chevron_path(wh: Wh<Px>) -> Path {
    Path::new()
        .move_to(0.px(), 0.px())
        .line_to(wh.width * 0.5, wh.height)
        .line_to(wh.width, 0.px())
}

fn damage_diamond_path(wh: Wh<Px>) -> Path {
    let half_w = wh.width * 0.5;
    let half_h = wh.height * 0.5;
    Path::new().add_poly(
        &[
            Xy::new(half_w, 0.px()),
            Xy::new(wh.width, half_h),
            Xy::new(half_w, wh.height),
            Xy::new(0.px(), half_h),
        ],
        true,
    )
}

fn damage_asterisk_path(wh: Wh<Px>) -> Path {
    let center = Xy::new(wh.width * 0.5, wh.height * 0.5);
    let outer_radius = wh.width.min(wh.height) * 0.5;
    let inner_radius = outer_radius * 0.5;
    let mut points = Vec::with_capacity(24);
    for i in 0..24 {
        let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / 12.0;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        points.push(Xy::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    Path::new().add_poly(&points, true)
}

fn damage_star_path(wh: Wh<Px>) -> Path {
    let center = Xy::new(wh.width * 0.5, wh.height * 0.5);
    let outer_radius = wh.width.min(wh.height) * 0.5;
    let inner_radius = outer_radius * 0.45;
    let mut points = Vec::with_capacity(10);
    for i in 0..10 {
        let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / 5.0;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        points.push(Xy::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    Path::new().add_poly(&points, true)
}
