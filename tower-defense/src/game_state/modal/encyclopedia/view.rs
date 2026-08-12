use super::catalog::{EntryKind, all_entries};
use super::entry_rendering::{CollectionStats, is_discovered, render_entries};
use super::layout::{
    CLOSE_VISIBLE_HEIGHT, CONTENT_PANEL_BOTTOM, CONTENT_PANEL_PADDING, CONTENT_PANEL_X,
    CONTENT_PANEL_Y, ENTRY_SIZE, FAB_PADDING, FAB_SIZE, PADDING, SCROLL_BAR_WIDTH,
    calculate_layout,
};
use super::tab_bar::{calculate_tab_bar_layout, render_tab_bar};
use crate::game_state::set_overlay_modal;
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use namui::*;
use namui_prebuilt::{scroll_view::ScrollViewWithCtx, simple_rect};

pub struct EncyclopediaModal;

impl Component for EncyclopediaModal {
    fn render(self, ctx: &RenderCtx) {
        let game_state = crate::game_state::use_game_state(ctx);
        let discovered = ctx.track_eq(&game_state.discovery);
        let screen_wh = screen::size().into_type::<Px>();
        let tracked_screen_wh = ctx.track_eq(&screen_wh);
        let screen_wh = *tracked_screen_wh;
        let entries = ctx.memo(all_entries);
        let (selected_kind, set_selected_kind) = ctx.state(|| EntryKind::Item);
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());
        let discovered_entries = ctx.memo(|| {
            println!("memoizing discovered entries");
            let discovered = discovered.as_ref();
            entries
                .iter()
                .map(|entry| is_discovered(entry, discovered))
                .collect::<Vec<_>>()
        });
        let stats = ctx.memo(|| {
            CollectionStats::from_entries(entries.as_slice(), discovered_entries.as_slice())
        });
        let content_panel_wh = Wh::new(
            (screen_wh.width - CONTENT_PANEL_X * 2.0).max(px(0.0)),
            (screen_wh.height - CONTENT_PANEL_Y - CONTENT_PANEL_BOTTOM).max(px(0.0)),
        );
        let content_wh = Wh::new(
            (content_panel_wh.width - CONTENT_PANEL_PADDING * 2.0).max(px(0.0)),
            (content_panel_wh.height - CONTENT_PANEL_PADDING * 2.0).max(px(0.0)),
        );
        let layout = ctx.memo(|| {
            let content_panel_width =
                (tracked_screen_wh.width - CONTENT_PANEL_X * 2.0).max(px(0.0));
            let content_width =
                (content_panel_width - CONTENT_PANEL_PADDING * 2.0 - SCROLL_BAR_WIDTH)
                    .max(ENTRY_SIZE + PADDING * 2.0);
            calculate_layout(entries.as_slice(), content_width, *selected_kind)
        });
        let tab_layout = ctx.memo(|| calculate_tab_bar_layout(*tracked_screen_wh));
        let locale = game_state.text().locale();
        let selected_kind_value = *selected_kind;

        ctx.translate((
            CONTENT_PANEL_X + CONTENT_PANEL_PADDING,
            CONTENT_PANEL_Y + CONTENT_PANEL_PADDING,
        ))
        .add(ScrollViewWithCtx {
            wh: content_wh,
            scroll_bar_width: SCROLL_BAR_WIDTH,
            content: |ctx| {
                render_entries(
                    &ctx,
                    entries.as_slice(),
                    layout.as_ref(),
                    discovered_entries.as_slice(),
                    stats.as_ref(),
                    locale,
                )
            },
            scroll_y: *scroll_y,
            set_scroll_y,
        })
        .attach_event(|event| match event {
            Event::MouseDown { event } | Event::MouseMove { event } | Event::MouseUp { event }
                if event.is_local_xy_in() =>
            {
                event.stop_propagation()
            }
            Event::Wheel { event } if event.is_local_xy_in() => event.stop_propagation(),
            _ => {}
        });

        ctx.translate((CONTENT_PANEL_X, CONTENT_PANEL_Y))
            .add(PaperContainerBackground {
                width: content_panel_wh.width,
                height: content_panel_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Paper,
                color: palette::SURFACE_CONTAINER_HIGH,
                outline_color: Some(palette::OUTLINE.with_alpha(180)),
                shadow: true,
                arrow: None,
            });

        ctx.compose(|ctx| {
            ctx.translate((screen_wh.width - FAB_PADDING - FAB_SIZE, FAB_PADDING))
                .add(
                    Button::new(
                        Wh::single(FAB_SIZE),
                        &|| set_overlay_modal(None),
                        &|wh, _color, ctx| {
                            let icon_wh = Wh::single(px(44.0));
                            ctx.translate((
                                (wh.width - icon_wh.width) * 0.5,
                                (CLOSE_VISIBLE_HEIGHT - icon_wh.height) * 0.5,
                            ))
                            .add(Icon {
                                kind: IconKind::Reject,
                                size: IconSize::Custom {
                                    size: icon_wh.width,
                                },
                                attributes: vec![],
                                wh: icon_wh,
                                opacity: 1.0,
                            });
                            ctx.add(PaperContainerBackground {
                                width: wh.width,
                                height: wh.height,
                                texture: PaperTexture::Rough,
                                variant: PaperVariant::PaperSingleLayer,
                                color: palette::SURFACE_CONTAINER_HIGH,
                                outline_color: None,
                                shadow: true,
                                arrow: None,
                            });
                        },
                    )
                    .variant(ButtonVariant::Text),
                );
        });

        ctx.compose(|ctx| {
            render_tab_bar(
                &ctx,
                tab_layout.as_ref(),
                selected_kind_value,
                set_selected_kind,
                set_scroll_y,
                locale,
            );
        });

        ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::BLACK.with_alpha(180),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
    }
}
