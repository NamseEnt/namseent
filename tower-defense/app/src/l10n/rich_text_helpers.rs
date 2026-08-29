use crate::card::Rank;
use crate::icon::IconKind;
use crate::theme::typography::TypographyBuilder;
use crate::theme::{self, palette};

/// Typography Builder extension trait for rich text helpers
pub trait RichTextHelpers<'a> {
    fn with_percentage_increase<S: Into<String>>(&mut self, value: S)
    -> &mut TypographyBuilder<'a>;
    fn with_percentage_decrease<S: Into<String>>(&mut self, value: S)
    -> &mut TypographyBuilder<'a>;
    fn with_card_rank(&mut self, rank: Rank) -> &mut TypographyBuilder<'a>;
    fn with_bold<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_icon_bold<S: Into<String>>(
        &mut self,
        icon_kind: IconKind,
        value: S,
    ) -> &mut TypographyBuilder<'a>;
}

impl<'a> RichTextHelpers<'a> for TypographyBuilder<'a> {
    fn with_percentage_increase<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.color(palette::COMMON).text(format!("+{}%", value.into()));
        });
        self
    }

    fn with_percentage_decrease<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.color(palette::RED).text(format!("-{}%", value.into()));
        });
        self
    }

    fn with_card_rank(&mut self, rank: Rank) -> &mut TypographyBuilder<'a> {
        self.card_rank(rank)
    }

    fn with_bold<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.bold()
                .color(theme::palette::BRIGHT_ORANGE)
                .text(value.into());
        });
        self
    }

    fn with_icon_bold<S: Into<String>>(
        &mut self,
        icon_kind: IconKind,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.icon(icon_kind);
        self.with_style(|b| {
            b.bold().static_text(" ").text(value.into());
        });
        self
    }
}
