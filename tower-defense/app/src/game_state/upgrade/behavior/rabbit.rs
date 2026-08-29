use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct RabbitUpgrade;

impl UpgradePresentation for RabbitUpgrade {
    fn key(&self) -> &'static str {
        "rabbit"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::RABBIT)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Rabbit",
            crate::l10n::locale::Language::Korean => "토끼",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Skip one rank in a straight",
            crate::l10n::locale::Language::Korean => {
                "스트레이트를 만들 때 하나를 건너뛸 수 있습니다"
            }
        });
    }
}

impl RabbitUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Rabbit(RabbitUpgrade)
    }
}
