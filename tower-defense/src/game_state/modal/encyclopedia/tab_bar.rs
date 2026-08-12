use super::catalog::EntryKind;
use super::layout::{
    FAB_PADDING, FAB_SIZE, TAB_BAR_HEIGHT, TAB_BAR_Y, TAB_CONTENT_TITLE_HEIGHT, TAB_GAP, TAB_LEFT,
    TAB_MAX_WIDTH, TAB_MIN_WIDTH, TAB_VISIBLE_HEIGHT, TabBarLayout,
};
use crate::l10n::ui::EncyclopediaText;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::{
    palette,
    typography::{FontSize, memoized_text},
};
use namui::*;

pub(super) fn render_tab_bar(
    ctx: &ComposeCtx,
    tab_layout: &TabBarLayout,
    selected_kind: EntryKind,
    set_selected_kind: SetState<EntryKind>,
    set_scroll_y: SetState<Px>,
    locale: crate::l10n::Locale,
) {
    let tab_width = tab_layout.tab_width;

    for (index, kind) in [EntryKind::Item, EntryKind::CardService, EntryKind::Treasure]
        .into_iter()
        .enumerate()
    {
        let active = selected_kind == kind;
        let on_click = || {
            set_selected_kind.set(kind);
            set_scroll_y.set(0.px());
        };
        let category_text = category_text(kind);
        let tab_color = if active {
            palette::SURFACE_CONTAINER_HIGH
        } else {
            palette::SURFACE_CONTAINER_LOW
        };

        ctx.translate((TAB_LEFT + (tab_width + TAB_GAP) * index as f32, TAB_BAR_Y))
            .add(
                Button::new(
                    Wh::new(tab_width, TAB_BAR_HEIGHT),
                    &on_click,
                    &move |_, _, ctx| {
                        let content_top =
                            (TAB_VISIBLE_HEIGHT - TAB_CONTENT_TITLE_HEIGHT).max(px(0.0)) * 0.5;
                        ctx.translate((px(0.0), content_top)).add(memoized_text(
                            (&kind, &locale),
                            |mut builder| {
                                builder
                                    .headline()
                                    .bold()
                                    .size(FontSize::Medium)
                                    .color(palette::ON_SURFACE)
                                    .stroke(1.px(), palette::SURFACE_CONTAINER_HIGHEST)
                                    .l10n(category_text, &locale)
                                    .render_center(Wh::new(tab_width, TAB_CONTENT_TITLE_HEIGHT))
                            },
                        ));
                        ctx.add(PaperContainerBackground {
                            width: tab_width,
                            height: TAB_BAR_HEIGHT,
                            texture: PaperTexture::Rough,
                            variant: PaperVariant::PaperSingleLayer,
                            color: tab_color,
                            outline_color: None,
                            shadow: true,
                            arrow: None,
                        });
                    },
                )
                .variant(ButtonVariant::Text),
            );
    }
}

pub(super) fn calculate_tab_bar_layout(screen_wh: Wh<Px>) -> TabBarLayout {
    let fab_reserved = FAB_SIZE + FAB_PADDING * 2.0;
    let tab_area_width = (screen_wh.width - TAB_LEFT - fab_reserved - TAB_GAP * 2.0)
        .max(TAB_MIN_WIDTH * 3.0 + TAB_GAP * 2.0);

    TabBarLayout {
        tab_width: (tab_area_width / 3.0).clamp(TAB_MIN_WIDTH, TAB_MAX_WIDTH),
    }
}

fn category_text(kind: EntryKind) -> EncyclopediaText {
    match kind {
        EntryKind::Item => EncyclopediaText::Items,
        EntryKind::CardService => EncyclopediaText::CardServices,
        EntryKind::Treasure => EncyclopediaText::Treasures,
    }
}
