use crate::{
    game_state::effect::Effect,
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    thumbnail::ThumbnailComposer,
};
use namui::*;

impl Effect {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            Effect::Heal { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Health)
                .build(),
            Effect::Lottery { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::ExtraReroll => Icon::new(IconKind::Refresh)
                .wh(width_height)
                .size(IconSize::Custom {
                    size: width_height.width,
                })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            Effect::Shield { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Shield)
                .build(),
            Effect::EarnGold { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::Gold)
                .build(),
            Effect::DamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
            Effect::UserDamageReduction { .. } => ThumbnailComposer::new(width_height)
                .with_icon_base(IconKind::AttackDamage)
                .build(),
        }
    }
}
