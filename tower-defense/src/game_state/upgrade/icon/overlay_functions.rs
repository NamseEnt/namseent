use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    theme::typography::{FontSize, TextAlign, headline},
};
use namui::*;

use super::constants::{LARGE_OVERLAY_SIZE_RATIO, OVERLAY_SIZE_RATIO, RANK_OVERLAY_SIZE_RATIO};

pub fn render_plus_overlay(wh: Wh<Px>) -> RenderingTree {
    let overlay_size = wh * OVERLAY_SIZE_RATIO;
    let overlay_xy = Xy::new(
        wh.width - overlay_size.width,
        wh.height - overlay_size.height,
    );

    namui::translate(
        overlay_xy.x,
        overlay_xy.y,
        Icon::new(IconKind::Add)
            .wh(overlay_size)
            .size(IconSize::Custom {
                size: overlay_size.width,
            })
            .to_rendering_tree(),
    )
}

pub fn render_rank_overlay(wh: Wh<Px>, rank: Rank) -> RenderingTree {
    let overlay_xy = Xy::new(
        wh.width - (wh.width * RANK_OVERLAY_SIZE_RATIO),
        wh.height - (wh.height * RANK_OVERLAY_SIZE_RATIO),
    );
    let overlay_size = wh * RANK_OVERLAY_SIZE_RATIO;

    namui::translate(
        overlay_xy.x,
        overlay_xy.y,
        namui::render([
            headline(rank.to_string())
                .align(TextAlign::Center { wh: overlay_size })
                .size(FontSize::Custom {
                    size: overlay_size.height * 0.6,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: overlay_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(0, 0, 0, 180),
                    }),
                    round: Some(RectRound {
                        radius: overlay_size.width * 0.5,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_suit_overlay(wh: Wh<Px>, suit: Suit) -> RenderingTree {
    let overlay_xy = Xy::new(
        wh.width - (wh.width * RANK_OVERLAY_SIZE_RATIO),
        wh.height - (wh.height * RANK_OVERLAY_SIZE_RATIO),
    );
    let overlay_size = wh * RANK_OVERLAY_SIZE_RATIO;

    namui::translate(
        overlay_xy.x,
        overlay_xy.y,
        Icon::new(IconKind::Suit { suit })
            .wh(overlay_size)
            .size(IconSize::Custom {
                size: overlay_size.width,
            })
            .to_rendering_tree(),
    )
}

pub fn render_expansion_overlay(wh: Wh<Px>, text: &str) -> RenderingTree {
    let overlay_size = wh * LARGE_OVERLAY_SIZE_RATIO;
    let overlay_xy = Xy::new(0.px(), wh.height - overlay_size.height);

    namui::translate(
        overlay_xy.x,
        overlay_xy.y,
        namui::render([
            headline(text)
                .align(TextAlign::Center { wh: overlay_size })
                .size(FontSize::Custom {
                    size: overlay_size.height * 0.4,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: overlay_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(0, 191, 255, 200), // 하늘색
                    }),
                    round: Some(RectRound {
                        radius: overlay_size.width * 0.2,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_low_card_indicator(wh: Wh<Px>) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), wh.height - indicator_size.height);

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline("≤3")
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.6,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(255, 105, 180, 200), // 핫핑크
                    }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.2,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_no_reroll_indicator(wh: Wh<Px>) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), wh.height - indicator_size.height);

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        Icon::new(IconKind::Refresh)
            .wh(indicator_size)
            .size(IconSize::Custom {
                size: indicator_size.width,
            })
            .attributes(vec![
                IconAttribute::new(IconKind::Reject).position(IconAttributePosition::Center),
            ])
            .to_rendering_tree(),
    )
}

pub fn render_reroll_indicator(wh: Wh<Px>) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), wh.height - indicator_size.height);

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        Icon::new(IconKind::Refresh)
            .wh(indicator_size)
            .size(IconSize::Custom {
                size: indicator_size.width,
            })
            .attributes(vec![
                IconAttribute::new(IconKind::Accept).position(IconAttributePosition::Center),
            ])
            .to_rendering_tree(),
    )
}

pub fn render_even_odd_indicator(wh: Wh<Px>, is_even: bool) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), wh.height - indicator_size.height);
    let text = if is_even { "Even" } else { "Odd" };
    let color = if is_even {
        Color::from_u8(70, 130, 180, 200) // 스틸블루
    } else {
        Color::from_u8(255, 69, 0, 200) // 오렌지레드
    };

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline(text)
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.4,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill { color }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.2,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_face_number_indicator(wh: Wh<Px>, is_face: bool) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), wh.height - indicator_size.height);
    let text = if is_face { "Face" } else { "Num" };
    let color = if is_face {
        Color::from_u8(148, 0, 211, 200) // 다크바이올렛
    } else {
        Color::from_u8(34, 139, 34, 200) // 포레스트그린
    };

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline(text)
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.4,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill { color }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.2,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_shortcut_indicator(wh: Wh<Px>, text: &str) -> RenderingTree {
    let indicator_size = wh * LARGE_OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), 0.px());

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline(text)
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.6,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(255, 20, 147, 200), // 딥핑크
                    }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.5,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_skip_indicator(wh: Wh<Px>) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), 0.px());

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline("Skip")
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.4,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(255, 165, 0, 200), // 오렌지
                    }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.2,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}

pub fn render_same_suits_indicator(wh: Wh<Px>) -> RenderingTree {
    let indicator_size = wh * OVERLAY_SIZE_RATIO;
    let indicator_xy = Xy::new(0.px(), 0.px());

    namui::translate(
        indicator_xy.x,
        indicator_xy.y,
        namui::render([
            headline("=")
                .align(TextAlign::Center { wh: indicator_size })
                .size(FontSize::Custom {
                    size: indicator_size.height * 0.8,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: indicator_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(106, 90, 205, 200), // 슬레이트블루
                    }),
                    round: Some(RectRound {
                        radius: indicator_size.width * 0.5,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}
