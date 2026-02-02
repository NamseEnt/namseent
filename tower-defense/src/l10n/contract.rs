use super::effect::EffectText;
use super::{Language, Locale, LocalizedText};
use crate::game_state::contract::ContractEffect;
use crate::rarity::Rarity;
use crate::theme::palette;
use crate::theme::typography::TypographyBuilder;

pub enum ContractText<'a> {
    Risk(&'a ContractEffect),
    Reward(&'a ContractEffect),
}

impl LocalizedText for ContractText<'_> {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl<'a> ContractText<'a> {
    fn apply_korean<'b>(self, builder: &mut TypographyBuilder<'b>) {
        match self {
            ContractText::Risk(ce) => {
                let phase_text = phase_ko(ce);
                builder
                    .with_style(|builder| {
                        builder.color(palette::RED).static_text("리스크: ");
                    })
                    .text(phase_text)
                    .static_text(" ");
                apply_effect_suffix_ko(builder, ce);
            }
            ContractText::Reward(ce) => {
                let phase_text = phase_ko(ce);
                builder
                    .with_style(|builder| {
                        builder.color(palette::BLUE).static_text("리턴: ");
                    })
                    .text(phase_text)
                    .static_text(" ");
                apply_effect_suffix_ko(builder, ce);
            }
        }
    }

    fn apply_english<'b>(self, builder: &mut TypographyBuilder<'b>) {
        match self {
            ContractText::Risk(ce) => {
                let phase_text = phase_en(ce);
                builder
                    .with_style(|builder| {
                        builder.color(palette::RED).static_text("Risk: ");
                    })
                    .text(phase_text)
                    .static_text(" ");
                apply_effect_suffix_en(builder, ce);
            }
            ContractText::Reward(ce) => {
                let phase_text = phase_en(ce);
                builder
                    .with_style(|builder| {
                        builder.color(palette::BLUE).static_text("Return: ");
                    })
                    .text(phase_text)
                    .static_text(" ");
                apply_effect_suffix_en(builder, ce);
            }
        }
    }
}

// 계약 이름 (희귀도 기반) l10n
pub enum ContractNameText {
    Rarity(Rarity),
}

impl LocalizedText for ContractNameText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl ContractNameText {
    pub fn to_korean(&self) -> &'static str {
        match self {
            ContractNameText::Rarity(Rarity::Common) => "일반 계약",
            ContractNameText::Rarity(Rarity::Rare) => "희귀 계약",
            ContractNameText::Rarity(Rarity::Epic) => "에픽 계약",
            ContractNameText::Rarity(Rarity::Legendary) => "전설 계약",
        }
    }

    pub fn to_english(&self) -> &'static str {
        match self {
            ContractNameText::Rarity(Rarity::Common) => "Common Contract",
            ContractNameText::Rarity(Rarity::Rare) => "Rare Contract",
            ContractNameText::Rarity(Rarity::Epic) => "Epic Contract",
            ContractNameText::Rarity(Rarity::Legendary) => "Legendary Contract",
        }
    }

    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        builder.static_text(self.to_korean());
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        builder.static_text(self.to_english());
    }
}

pub enum ContractDurationText<'a> {
    Status(&'a crate::game_state::contract::ContractStatus),
}

impl LocalizedText for ContractDurationText<'_> {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl ContractDurationText<'_> {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            ContractDurationText::Status(status) => {
                use crate::game_state::contract::ContractStatus;
                if let ContractStatus::Pending { duration_stages } = status {
                    builder
                        .text(format!("{}", duration_stages))
                        .static_text("스테이지동안 지속됩니다");
                }
            }
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            ContractDurationText::Status(status) => {
                use crate::game_state::contract::ContractStatus;
                if let ContractStatus::Pending { duration_stages } = status {
                    builder
                        .static_text("for ")
                        .text(format!("{}", duration_stages))
                        .static_text(" stages");
                }
            }
        }
    }
}

pub fn duration_korean(status: &crate::game_state::contract::ContractStatus) -> String {
    use crate::game_state::contract::ContractStatus;
    match status {
        ContractStatus::Pending { duration_stages } => {
            format!("{duration_stages}스테이지동안 지속됩니다")
        }
        _ => String::new(),
    }
}

pub fn duration_english(status: &crate::game_state::contract::ContractStatus) -> String {
    use crate::game_state::contract::ContractStatus;
    match status {
        ContractStatus::Pending { duration_stages } => format!("for {duration_stages} stages"),
        _ => String::new(),
    }
}

fn phase_ko(ce: &ContractEffect) -> String {
    match ce {
        ContractEffect::OnSign { .. } => "계약 시".into(),
        ContractEffect::WhileActive { .. } => "계약 중".into(),
        ContractEffect::OnStageStart { .. } => "매 스테이지".into(),
        ContractEffect::OnExpire { .. } => "계약 종료 시".into(),
    }
}

fn phase_en(ce: &ContractEffect) -> String {
    match ce {
        ContractEffect::OnSign { .. } => "On sign".into(),
        ContractEffect::WhileActive { .. } => "While active".into(),
        ContractEffect::OnStageStart { .. } => "On stage start".into(),
        ContractEffect::OnExpire { .. } => "On expire".into(),
    }
}

fn apply_effect_suffix_ko<'a>(builder: &mut TypographyBuilder<'a>, ce: &ContractEffect) {
    let eff = match ce {
        ContractEffect::OnSign { effect }
        | ContractEffect::WhileActive { effect }
        | ContractEffect::OnStageStart { effect }
        | ContractEffect::OnExpire { effect } => effect,
    };
    builder
        .static_text("· ")
        .l10n(EffectText::Description(eff.clone()), &Locale::KOREAN);
}

fn apply_effect_suffix_en<'a>(builder: &mut TypographyBuilder<'a>, ce: &ContractEffect) {
    let eff = match ce {
        ContractEffect::OnSign { effect }
        | ContractEffect::WhileActive { effect }
        | ContractEffect::OnStageStart { effect }
        | ContractEffect::OnExpire { effect } => effect,
    };
    builder
        .static_text(" ")
        .l10n(EffectText::Description(eff.clone()), &Locale::ENGLISH);
}
