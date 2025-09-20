use super::effect::EffectText;
use crate::game_state::contract::ContractEffect;

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
