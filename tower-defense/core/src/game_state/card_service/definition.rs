use super::behaviors::{
    CardServiceBehavior, CardServiceBehaviorImpl, battery, brush, cactus, club_sword, copier,
    eraser, fountain_pen, long_sword, mace, magic_wand, magnet, pliers, screwdriver, spinning_top,
    staff, tricycle,
};

pub(crate) static CARD_SERVICE_BEHAVIORS: [CardServiceBehaviorImpl; crate::CardServiceKind::COUNT] = [
    CardServiceBehaviorImpl::LongSword(long_sword::Behavior),
    CardServiceBehaviorImpl::Staff(staff::Behavior),
    CardServiceBehaviorImpl::Mace(mace::Behavior),
    CardServiceBehaviorImpl::ClubSword(club_sword::Behavior),
    CardServiceBehaviorImpl::Brush(brush::Behavior),
    CardServiceBehaviorImpl::FountainPen(fountain_pen::Behavior),
    CardServiceBehaviorImpl::Tricycle(tricycle::Behavior),
    CardServiceBehaviorImpl::Eraser(eraser::Behavior),
    CardServiceBehaviorImpl::MagicWand(magic_wand::Behavior),
    CardServiceBehaviorImpl::Pliers(pliers::Behavior),
    CardServiceBehaviorImpl::Screwdriver(screwdriver::Behavior),
    CardServiceBehaviorImpl::Copier(copier::Behavior),
    CardServiceBehaviorImpl::Magnet(magnet::Behavior),
    CardServiceBehaviorImpl::Cactus(cactus::Behavior),
    CardServiceBehaviorImpl::SpinningTop(spinning_top::Behavior),
    CardServiceBehaviorImpl::Battery(battery::Behavior),
];

pub(crate) fn card_service_behavior(
    kind: crate::CardServiceKind,
) -> Option<&'static CardServiceBehaviorImpl> {
    CARD_SERVICE_BEHAVIORS
        .get(usize::from(kind.raw()))
        .filter(|behavior| behavior.kind() == kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_exactly_one_definition_for_each_card_service_kind() {
        assert_eq!(
            CARD_SERVICE_BEHAVIORS.len(),
            crate::CardServiceKind::ALL.len()
        );
        for &kind in crate::CardServiceKind::ALL {
            let matches = CARD_SERVICE_BEHAVIORS
                .iter()
                .filter(|behavior| behavior.kind() == kind)
                .count();
            assert_eq!(matches, 1, "card service kind {:?}", kind);
            assert_eq!(
                card_service_behavior(kind).map(|behavior| behavior.kind()),
                Some(kind)
            );
        }
    }
}
