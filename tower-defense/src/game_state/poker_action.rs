use crate::l10n::Locale;
use namui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub enum PokerAction {
    Fold,
    Call,
    Raise,
    AllIn,
}

impl PokerAction {
    pub fn dopamine_delta(self) -> i8 {
        match self {
            PokerAction::Fold => -2,
            PokerAction::Call => -1,
            PokerAction::Raise => 1,
            PokerAction::AllIn => 1,
        }
    }

    pub fn token_delta(self) -> i8 {
        match self {
            PokerAction::Fold | PokerAction::Call => 0,
            PokerAction::Raise | PokerAction::AllIn => 1,
        }
    }

    pub fn is_boss_only(self) -> bool {
        matches!(self, PokerAction::AllIn)
    }

    pub fn to_text(self, locale: &Locale) -> &'static str {
        match locale.language {
            crate::l10n::Language::Korean => match self {
                PokerAction::Fold => "폴드",
                PokerAction::Call => "콜",
                PokerAction::Raise => "레이즈",
                PokerAction::AllIn => "올인",
            },
            crate::l10n::Language::English => match self {
                PokerAction::Fold => "Fold",
                PokerAction::Call => "Call",
                PokerAction::Raise => "Raise",
                PokerAction::AllIn => "All-in",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub enum NextStageOffer {
    None,
    Shop,
    TreasureSelection,
}

pub fn roll_call_offer(rng: &mut impl rand::Rng) -> NextStageOffer {
    let roll = rng.gen_range(0..100);
    if roll < 25 {
        NextStageOffer::None
    } else if roll < 90 {
        NextStageOffer::Shop
    } else {
        NextStageOffer::TreasureSelection
    }
}
