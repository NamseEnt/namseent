mod description;
mod name;

use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum Word {
    Health,
    Gold,
    Dice,
    Item,
    Treasure,
    Shield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct WordName(pub Word);

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct WordDescription(pub Word);

impl Word {
    pub fn name(&self) -> WordName {
        WordName(*self)
    }

    pub fn description(&self) -> WordDescription {
        WordDescription(*self)
    }
}
