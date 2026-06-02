use super::*;

use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct PainkillerItem {
    pub damage_multiply: f32,
    pub duration: Duration,
}

impl PainkillerItem {
    pub fn new(damage_multiply: f32, duration: Duration) -> Self {
        Self {
            damage_multiply,
            duration,
        }
    }

    pub fn standard() -> Self {
        Self::new(0.85, Duration::from_secs(4))
    }

    pub fn into_item(self) -> Item {
        Item::Painkiller(self)
    }
}

impl ItemBehavior for PainkillerItem {
    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        let status_effect = crate::game_state::user_status_effect::UserStatusEffect {
            kind: crate::game_state::user_status_effect::UserStatusEffectKind::DamageReduction {
                damage_multiply: self.damage_multiply,
            },
            end_at: game_state.now() + self.duration,
        };
        game_state.action(crate::game_state::GameStateAction::ApplyUserStatusEffect(
            status_effect,
        ));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "진통제",
            crate::l10n::Language::English => "Painkiller",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        let reduction_percent = (1.0 - self.damage_multiply) * 100.0;
        let duration_secs = self.duration.as_secs_f32();
        match locale.language {
            crate::l10n::Language::Korean => {
                builder
                    .with_icon_bold(IconKind::Damage, format!("-{reduction_percent:.0}%"))
                    .static_text(" 피해 ")
                    .with_time_duration(format!("{duration_secs:.0}초"))
                    .static_text(" 감소");
            }
            crate::l10n::Language::English => {
                builder
                    .with_icon_bold(IconKind::Damage, format!("-{reduction_percent:.0}%"))
                    .static_text(" damage for ")
                    .with_time_duration(format!("{duration_secs:.0}s"));
            }
        }
    }

    fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        render_sticker(
            crate::asset::image::thumbnail::PAINKILLER,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
