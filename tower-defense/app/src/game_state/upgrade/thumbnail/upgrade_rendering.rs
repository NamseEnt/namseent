use crate::game_state::upgrade::Upgrade;
use crate::game_state::upgrade::behavior::UpgradePresentation;

impl Upgrade {
    pub fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        UpgradePresentation::thumbnail_source(self)
    }

    pub fn thumbnail_overlays(
        &self,
        game_state: &crate::game_state::GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        UpgradePresentation::thumbnail_overlays(self, game_state)
    }
}
