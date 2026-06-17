use crate::{l10n::word::Word, tooltip::SectionText};

impl Word {
    pub(super) fn tooltip_sections(
        self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'static>> {
        match self {
            Word::Health => vec![self.tooltip_section(locale)],
            Word::Gold => vec![self.tooltip_section(locale)],
            Word::Dice => vec![self.tooltip_section(locale)],
            Word::Item => vec![self.tooltip_section(locale)],
            Word::Treasure => vec![self.tooltip_section(locale)],
            Word::Shield => vec![self.tooltip_section(locale)],
            Word::PerfectClear => vec![self.tooltip_section(locale)],
        }
    }

    pub fn tooltip_section(
        self,
        locale: crate::l10n::Locale,
    ) -> crate::tooltip::TooltipSection<'static> {
        crate::tooltip::TooltipSection {
            title: Some(SectionText {
                key: format!("word:{self:?}:name"),
                apply: Box::new(move |builder| {
                    builder.l10n(self.name(), &locale);
                }),
            }),
            body: SectionText {
                key: format!("word:{self:?}:desc"),
                apply: Box::new(move |builder| {
                    builder.l10n(self.description(), &locale);
                }),
            },
        }
    }
}
