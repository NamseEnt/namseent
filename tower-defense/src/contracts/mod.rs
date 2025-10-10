mod contract_item;

use crate::{
    game_state::{
        GameState,
        contract::ContractId,
        flow::{GameFlow, contract::ContractFlowState},
        use_game_state,
    },
    icon::{Icon, IconKind, IconSize},
    l10n::TextManager,
    palette,
};
use contract_item::ContractItemContent;
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

const PANEL_WIDTH: Px = px(260.);
const PADDING: Px = px(4.);
const TITLE_HEIGHT: Px = px(36.);

pub struct ContractsPanel {
    pub screen_wh: Wh<Px>,
}

impl Component for ContractsPanel {
    fn render(self, render_ctx: &RenderCtx) {
        let game_state = use_game_state(render_ctx);
        let text_manager: TextManager = game_state.text();
        let evaluating_contract_id = get_evaluating_contract_id(&game_state);

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.clip(Path::new().add_rect(wh.to_rect()), ClipOp::Intersect)
                .add(AutoScrollViewWithCtx {
                    wh,
                    scroll_bar_width: PADDING,
                    content: |ctx| {
                        let content_width = wh.width;
                        let mut current_y = 0.px();
                        for contract in game_state.contracts.iter() {
                            let item = ctx.ghost_compose("ContractItemContent", |ctx| {
                                ctx.add(ContractItemContent {
                                    contract,
                                    text_manager,
                                    content_width,
                                    evaluating_contract_id,
                                });
                            });
                            let Some(container_wh) = item.bounding_box().map(|rect| rect.wh())
                            else {
                                return;
                            };
                            ctx.translate((0.px(), current_y)).add(item);
                            current_y += container_wh.height;
                        }
                    },
                });
        };

        render_ctx.compose(|ctx| {
            table::horizontal([
                table::fixed_no_clip(
                    PANEL_WIDTH,
                    table::padding(
                        PADDING,
                        table::vertical([
                            table::fixed(TITLE_HEIGHT, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Quest).size(IconSize::Medium).wh(Wh {
                                    width: 32.px(),
                                    height: wh.height,
                                }));

                                ctx.add(rect(RectParam {
                                    rect: wh.to_rect(),
                                    style: RectStyle {
                                        stroke: Some(RectStroke {
                                            color: palette::OUTLINE,
                                            width: 1.px(),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        fill: Some(RectFill {
                                            color: palette::SURFACE_CONTAINER,
                                        }),
                                        round: Some(RectRound {
                                            radius: palette::ROUND,
                                        }),
                                    },
                                }));
                            }),
                            table::fixed_no_clip(PADDING, |_, _| {}),
                            table::ratio(1, scroll_view),
                        ]),
                    ),
                ),
                table::ratio_no_clip(1, |_, _| {}),
            ])(self.screen_wh, ctx);
        });
    }
}

pub struct Contracts {
    pub screen_wh: Wh<Px>,
}
impl Component for Contracts {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(ContractsPanel {
            screen_wh: self.screen_wh,
        });
    }
}

fn get_evaluating_contract_id(game_state: &GameState) -> Option<ContractId> {
    let GameFlow::Contract(contract_flow) = &game_state.flow else {
        return None;
    };
    match &contract_flow.state {
        ContractFlowState::Active { event, .. } | ContractFlowState::Standby { event, .. } => {
            Some(event.contract_id)
        }
        _ => None,
    }
}
