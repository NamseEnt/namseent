use super::constants::{PADDING, SHOP_REFRESH_BUTTON_WH, SHOP_WH};
use super::items::ShopItem;
use crate::flow_ui::selecting_tower::shop_modal::constants::SHOP_SLOT_WIDTH;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::hand::xy_with_spring;
use crate::icon::{Icon, IconKind, IconSize};
use crate::shop::refresh_shop;
use crate::shop::{Shop, ShopSlotId};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline};
use namui::*;
use namui_prebuilt::simple_rect;

pub struct ShopLayout<'a> {
    pub shop: &'a Shop,
    pub purchase_item: &'a dyn Fn(ShopSlotId),
    pub can_purchase_item: &'a dyn Fn(ShopSlotId) -> bool,
    pub button_xy: Xy<Px>,
}

impl Component for ShopLayout<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            shop,
            purchase_item,
            can_purchase_item,
            button_xy,
        } = self;

        let game_state = use_game_state(ctx);
        let disabled = game_state.left_shop_refresh_chance == 0 || {
            let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();
            (game_state.hp - health_cost as f32) < 1.0
        };

        let refresh_shop = || {
            mutate_game_state(|game_state| {
                let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();
                if (game_state.hp - health_cost as f32) < 1.0 {
                    return;
                }
                game_state.left_shop_refresh_chance -= 1;
                game_state.take_damage(health_cost as f32);
                refresh_shop(game_state);
            });
        };

        let (hovered_slot_id, set_hovered_slot_id) = ctx.state::<Option<ShopSlotId>>(|| None);

        // 절대 좌표 기반 레이아웃으로 전환
        ctx.compose(|ctx| {
            // 외곽 패딩 영역으로 기준 좌표 이동
            let content_wh = Wh {
                width: SHOP_WH.width - PADDING * 2.0,
                height: SHOP_WH.height - PADDING * 2.0,
            };
            let items_area_wh = Wh {
                width: content_wh.width,
                height: content_wh.height - SHOP_REFRESH_BUTTON_WH.height,
            };

            // 슬롯 레이아웃 계산 (Hand 스타일: 중앙 정렬, 초과 시 음수 갭로 겹치기)
            let slot_count = shop.slots.len();
            if slot_count > 0 {
                let n = slot_count as f32;
                // 고정 슬롯 폭(최대 items 영역 너비로 클램프)
                let slot_w = SHOP_SLOT_WIDTH.min(items_area_wh.width);
                let default_gap = PADDING;
                let total_with_default = slot_w * n + default_gap * (n - 1.0);
                let gap = if slot_count > 1 {
                    if total_with_default > items_area_wh.width {
                        // 초과 시 음수 갭
                        (items_area_wh.width - slot_w * n) / (n - 1.0)
                    } else {
                        default_gap
                    }
                } else {
                    px(0.0)
                };
                let total_width = slot_w * n + gap * (n - 1.0);
                let start_x = (items_area_wh.width - total_width) / 2.0;
                let slot_wh = Wh::new(slot_w, items_area_wh.height);

                if let Some(hovered_id) = *hovered_slot_id
                    && let Some((index, slot_data)) = shop
                        .slots
                        .iter()
                        .enumerate()
                        .find(|(_, slot)| slot.id == hovered_id)
                {
                    let x = start_x + (slot_w + gap) * index as f32;
                    let y = px(0.0);
                    let target_xy = Xy::new(x, y);

                    ctx.translate((PADDING, PADDING)).add_with_key(
                        hovered_id,
                        ShopSlotView {
                            wh: slot_wh,
                            slot_data,
                            purchase_item,
                            can_purchase_item: can_purchase_item(hovered_id),
                            target_xy,
                            hovered_slot_id: *hovered_slot_id,
                            set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                            button_xy,
                        },
                    );
                }

                for (index, slot_data) in shop.slots.iter().enumerate() {
                    let slot_id = slot_data.id;
                    if *hovered_slot_id == Some(slot_id) {
                        continue; // 호버된 슬롯은 건너뜀
                    }

                    let x = start_x + (slot_w + gap) * index as f32;
                    let y = px(0.0);
                    let target_xy = Xy::new(x, y);

                    ctx.translate((PADDING, PADDING)).add_with_key(
                        slot_id,
                        ShopSlotView {
                            wh: slot_wh,
                            slot_data,
                            purchase_item,
                            can_purchase_item: can_purchase_item(slot_id),
                            target_xy,
                            hovered_slot_id: *hovered_slot_id,
                            set_hovered_slot_id: &|id| set_hovered_slot_id.set(id),
                            button_xy,
                        },
                    );
                }
            }

            // 리롤 버튼을 하단 중앙에 배치 (절대 좌표)
            let btn_xy = Xy::new(
                (content_wh.width - SHOP_REFRESH_BUTTON_WH.width) * 0.5,
                items_area_wh.height,
            );
            ctx.translate((PADDING, PADDING)).translate(btn_xy).add(
                Button::new(
                    SHOP_REFRESH_BUTTON_WH,
                    &|| {
                        refresh_shop();
                    },
                    &|wh, color, ctx| {
                        let health_cost = game_state.stage_modifiers.get_shop_reroll_health_cost();
                        let mut text = format!(
                            "{}-{}",
                            Icon::new(IconKind::Refresh)
                                .size(IconSize::Large)
                                .wh(Wh::single(wh.height))
                                .as_tag(),
                            game_state.left_shop_refresh_chance
                        );
                        if health_cost > 0 {
                            text.push_str(&format!(
                                " {}",
                                Icon::new(IconKind::Health)
                                    .size(IconSize::Small)
                                    .wh(Wh::single(wh.height * 0.5))
                                    .as_tag()
                            ));
                        }
                        ctx.add(
                            headline(text)
                                .color(color)
                                .align(TextAlign::Center { wh })
                                .build_rich(),
                        );
                    },
                )
                .variant(ButtonVariant::Fab)
                .disabled(disabled),
            );
        });
    }
}

// 슬롯 단위 애니메이션 및 실제 아이템 렌더링을 담당하는 뷰
struct ShopSlotView<'a> {
    wh: Wh<Px>,
    slot_data: &'a crate::shop::ShopSlotData,
    purchase_item: &'a dyn Fn(ShopSlotId),
    can_purchase_item: bool,
    target_xy: Xy<Px>,
    hovered_slot_id: Option<ShopSlotId>,
    set_hovered_slot_id: &'a dyn Fn(Option<ShopSlotId>),
    button_xy: Xy<Px>,
}

impl Component for ShopSlotView<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            slot_data,
            purchase_item,
            can_purchase_item,
            target_xy,
            hovered_slot_id,
            set_hovered_slot_id,
            button_xy: _,
        } = self;

        let slot_id = slot_data.id;

        // Exit 애니메이션이 있는 경우 처리 (제자리에서 scale만 0으로)
        let (target_xy, target_scale) = if slot_data.exit_animation.is_some() {
            // 위치는 그대로, scale만 0으로
            (target_xy, Xy::single(0.0))
        } else {
            // 이 슬롯이 현재 호버된 슬롯인지 확인
            let hovering = hovered_slot_id == Some(slot_id);

            // 호버 시 1.2배 스케일
            let scale = if hovering {
                Xy::single(1.2)
            } else {
                Xy::single(1.0)
            };
            (target_xy, scale)
        };

        // 아래에서 위로 스르륵 올라오는 기본 진입 애니메이션
        let initial_xy = Xy::new(target_xy.x, target_xy.y + px(64.0));
        let animated_xy = xy_with_spring(ctx, target_xy, initial_xy);

        let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(0.0));

        let half_xy = wh.to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_xy)
            .scale(animated_scale)
            .translate(-half_xy);

        // Exit 애니메이션 중인지 확인
        let is_exiting = slot_data.exit_animation.is_some();
        let hovering = hovered_slot_id == Some(slot_id);

        // 실제 콘텐츠 렌더링
        ctx.compose(|ctx| {
            ctx.add(ShopItem {
                wh,
                slot_data,
                purchase_item,
                can_purchase_item,
            });

            // Exit 애니메이션 중이 아닐 때만 hover 감지
            if !is_exiting {
                ctx.add(
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                        move |event| {
                            let Event::MouseMove { event } = event else {
                                return;
                            };
                            if event.is_local_xy_in() {
                                set_hovered_slot_id(Some(slot_id));
                            } else if hovering {
                                // 현재 호버된 슬롯에서 마우스가 벗어났을 때만 None으로 설정
                                set_hovered_slot_id(None);
                            }
                        },
                    ),
                );
            }
        });
    }
}
