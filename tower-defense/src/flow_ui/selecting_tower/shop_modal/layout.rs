use super::constants::{PADDING, SHOP_REFRESH_BUTTON_WH, SHOP_WH};
use super::items::ShopItem;
use crate::game_state::{mutate_game_state, use_game_state};
use crate::hand::xy_with_spring;
use crate::icon::{Icon, IconKind, IconSize};
use crate::shop::refresh_shop;
use crate::shop::{Shop, ShopSlot};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline};
use namui::*;

pub struct ShopLayout<'a> {
    pub shop: &'a Shop,
    pub purchase_item: &'a dyn Fn(usize),
    pub can_purchase_items: &'a [bool],
}

impl Component for ShopLayout<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            shop,
            purchase_item,
            can_purchase_items,
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

            // 슬롯 레이아웃 계산 (가로 균등 배치 + PADDING 간격)
            let slot_count = shop.slots.len();
            if slot_count > 0 {
                let gap = PADDING;
                let slot_w =
                    (items_area_wh.width - gap * (slot_count as f32 - 1.0)) / slot_count as f32;
                let slot_wh = Wh::new(slot_w, items_area_wh.height);

                for (shop_slot_index, shop_slot) in shop.slots.iter().enumerate() {
                    let x = slot_w * shop_slot_index as f32 + gap * shop_slot_index as f32;
                    let y = px(0.0);
                    let target_xy = Xy::new(x, y);

                    // 각 슬롯을 키와 함께 추가하여 애니메이션 상태 유지
                    ctx.translate((PADDING, PADDING)).add_with_key(
                        AddKey::U128(shop_slot_index as u128),
                        ShopSlotView {
                            wh: slot_wh,
                            shop_slot,
                            shop_slot_index,
                            purchase_item,
                            can_purchase_item: can_purchase_items[shop_slot_index],
                            target_xy,
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
    shop_slot: &'a ShopSlot,
    shop_slot_index: usize,
    purchase_item: &'a dyn Fn(usize),
    can_purchase_item: bool,
    target_xy: Xy<Px>,
}

impl Component for ShopSlotView<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            shop_slot,
            shop_slot_index,
            purchase_item,
            can_purchase_item,
            target_xy,
        } = self;

        // 아래에서 위로 스르륵 올라오는 기본 진입 애니메이션
        let initial_xy = Xy::new(target_xy.x, target_xy.y + px(64.0));
        let animated_xy = xy_with_spring(ctx, target_xy, initial_xy);

        ctx.translate(animated_xy).add(ShopItem {
            wh,
            shop_slot,
            shop_slot_index,
            purchase_item,
            can_purchase_item,
        });
    }
}
