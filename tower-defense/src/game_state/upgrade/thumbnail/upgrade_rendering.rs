use crate::game_state::upgrade::Upgrade;
use crate::game_state::upgrade::behavior::UpgradeBehavior;

impl Upgrade {
    pub fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        UpgradeBehavior::thumbnail_source(self)
    }

    pub fn thumbnail_overlays(
        &self,
        game_state: &crate::game_state::GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        UpgradeBehavior::thumbnail_overlays(self, game_state)
    }
}
