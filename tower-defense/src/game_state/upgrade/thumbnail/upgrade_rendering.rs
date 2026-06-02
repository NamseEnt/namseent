use crate::game_state::upgrade::Upgrade;
use crate::game_state::upgrade::behavior::UpgradeBehavior;
use namui::*;

impl Upgrade {
    pub fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        UpgradeBehavior::thumbnail(self, width_height, shadow)
    }
}
