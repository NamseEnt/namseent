use super::effect::EffectText;
use crate::game_state::contract::ContractEffect;
use crate::rarity::Rarity;

pub enum ContractText<'a> {
    Risk(&'a ContractEffect),
    Reward(&'a ContractEffect),
}

impl<'a> ContractText<'a> {
    pub fn to_korean(&self) -> String {
        match self {
            ContractText::Risk(ce) => format!(
                "{} {} {}",
                super::rich_text_helpers::contract_risk("리스크:"),
                phase_ko(ce),
                effect_suffix_ko(ce)
            ),
            ContractText::Reward(ce) => format!(
                "{} {} {}",
                super::rich_text_helpers::contract_reward("리턴:"),
                phase_ko(ce),
                effect_suffix_ko(ce)
            ),
        }
    }

    pub fn to_english(&self) -> String {
        match self {
            ContractText::Risk(ce) => format!(
                "{} {} {}",
                super::rich_text_helpers::contract_risk("Risk:"),
                phase_en(ce),
                effect_suffix_en(ce)
            ),
            ContractText::Reward(ce) => format!(
                "{} {} {}",
                super::rich_text_helpers::contract_reward("Return:"),
                phase_en(ce),
                effect_suffix_en(ce)
            ),
        }
    }
}

// 계약 이름 (희귀도 기반) l10n
pub enum ContractNameText {
    Rarity(Rarity),
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

fn effect_suffix_ko(ce: &ContractEffect) -> String {
    let eff = match ce {
        ContractEffect::OnSign { effect }
        | ContractEffect::WhileActive { effect }
        | ContractEffect::OnStageStart { effect }
        | ContractEffect::OnExpire { effect } => effect,
    };
    let s = EffectText::Description(eff.clone()).to_korean();
    if s.is_empty() {
        String::new()
    } else {
        format!(" · {s}")
    }
}

fn effect_suffix_en(ce: &ContractEffect) -> String {
    let eff = match ce {
        ContractEffect::OnSign { effect }
        | ContractEffect::WhileActive { effect }
        | ContractEffect::OnStageStart { effect }
        | ContractEffect::OnExpire { effect } => effect,
    };
    let s = EffectText::Description(eff.clone()).to_english();
    if s.is_empty() {
        String::new()
    } else {
        format!(" · {s}")
    }
}
