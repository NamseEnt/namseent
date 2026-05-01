use super::*;

// Card hand rule modifiers
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FourLeafCloverUpgrade;
impl UpgradeBehavior for FourLeafCloverUpgrade {
    fn shorten_straight_flush_to_4_cards(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct RabbitUpgrade;
impl UpgradeBehavior for RabbitUpgrade {
    fn skip_rank_for_straight(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;
impl UpgradeBehavior for BlackWhiteUpgrade {
    fn treat_suits_as_same(&self) -> bool {
        true
    }
}
