use crate::{
    asset::image::thumbnail as thumbnail_image, game_state::upgrade::Upgrade,
    thumbnail::render_sticker_image_with_shadow,
};
use namui::*;

const UPGRADE_STICKER_THUMBNAIL_STROKE: Px = px(6.0);

fn sticker_thumbnail(image: Image, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
    render_sticker_image_with_shadow(
        image,
        width_height,
        UPGRADE_STICKER_THUMBNAIL_STROKE,
        shadow,
    )
}

impl Upgrade {
    pub fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        match self {
            Upgrade::Cat(..) => sticker_thumbnail(thumbnail_image::CAT, width_height, shadow),
            Upgrade::Staff(..) => sticker_thumbnail(thumbnail_image::STAFF, width_height, shadow),
            Upgrade::LongSword(..) => {
                sticker_thumbnail(thumbnail_image::LONG_SWORD, width_height, shadow)
            }
            Upgrade::Mace(..) => sticker_thumbnail(thumbnail_image::MACE, width_height, shadow),
            Upgrade::ClubSword(..) => {
                sticker_thumbnail(thumbnail_image::CLUB_SWORD, width_height, shadow)
            }
            Upgrade::Backpack(..) => {
                sticker_thumbnail(thumbnail_image::BACKPACK, width_height, shadow)
            }
            Upgrade::DiceBundle(..) => {
                sticker_thumbnail(thumbnail_image::DICE_BUNDLE, width_height, shadow)
            }
            Upgrade::Tricycle(..) => {
                sticker_thumbnail(thumbnail_image::TRICYCLE, width_height, shadow)
            }
            Upgrade::EnergyDrink(..) => {
                sticker_thumbnail(thumbnail_image::ENERGY_DRINK, width_height, shadow)
            }
            Upgrade::PerfectPottery(..) => {
                sticker_thumbnail(thumbnail_image::PERFECT_POTTERY, width_height, shadow)
            }
            Upgrade::SingleChopstick(..) => {
                sticker_thumbnail(thumbnail_image::SINGLE_CHOPSTICK, width_height, shadow)
            }
            Upgrade::PairChopsticks(..) => {
                sticker_thumbnail(thumbnail_image::PAIR_CHOPSTICK, width_height, shadow)
            }
            Upgrade::FountainPen(..) => {
                sticker_thumbnail(thumbnail_image::FOUNTAIN_PEN, width_height, shadow)
            }
            Upgrade::Brush(..) => sticker_thumbnail(thumbnail_image::BRUSH, width_height, shadow),
            Upgrade::FourLeafClover(..) => {
                sticker_thumbnail(thumbnail_image::FOUR_LEAF_CLOVER, width_height, shadow)
            }
            Upgrade::Rabbit(..) => sticker_thumbnail(thumbnail_image::RABBIT, width_height, shadow),
            Upgrade::BlackWhite(..) => {
                sticker_thumbnail(thumbnail_image::BLACK_WHITE, width_height, shadow)
            }
            Upgrade::Eraser(..) => sticker_thumbnail(thumbnail_image::ERASER, width_height, shadow),
            Upgrade::BrokenPottery(..) => {
                sticker_thumbnail(thumbnail_image::BROKEN_POTTERY, width_height, shadow)
            }
            Upgrade::Trophy(..) => sticker_thumbnail(thumbnail_image::TROPHY, width_height, shadow),
            Upgrade::Crock(..) => sticker_thumbnail(thumbnail_image::CROCK, width_height, shadow),
            Upgrade::DemolitionHammer(..) => {
                sticker_thumbnail(thumbnail_image::DEMOLITION_HAMMER, width_height, shadow)
            }
            Upgrade::Metronome(..) => {
                sticker_thumbnail(thumbnail_image::METRONOME, width_height, shadow)
            }
            Upgrade::Tape(..) => sticker_thumbnail(thumbnail_image::TAPE, width_height, shadow),
            Upgrade::NameTag(..) => {
                sticker_thumbnail(thumbnail_image::NAME_TAG, width_height, shadow)
            }
            Upgrade::ShoppingBag(..) => {
                sticker_thumbnail(thumbnail_image::SHOPPING_BAG, width_height, shadow)
            }
            Upgrade::Resolution(..) => {
                sticker_thumbnail(thumbnail_image::RESOLUTION, width_height, shadow)
            }
            Upgrade::Mirror(..) => sticker_thumbnail(thumbnail_image::MIRROR, width_height, shadow),
            Upgrade::IceCream(..) => {
                sticker_thumbnail(thumbnail_image::ICE_CREAM, width_height, shadow)
            }
            Upgrade::Spanner(..) => {
                sticker_thumbnail(thumbnail_image::SPANNER, width_height, shadow)
            }
            Upgrade::Pea(..) => sticker_thumbnail(thumbnail_image::PEA, width_height, shadow),
            Upgrade::SlotMachine(..) => {
                sticker_thumbnail(thumbnail_image::SLOT_MACHINE, width_height, shadow)
            }
            Upgrade::PiggyBank(..) => {
                sticker_thumbnail(thumbnail_image::PIGGY_BANK, width_height, shadow)
            }
            Upgrade::Camera(..) => sticker_thumbnail(thumbnail_image::CAMERA, width_height, shadow),
            Upgrade::GiftBox(..) => {
                sticker_thumbnail(thumbnail_image::GIFT_BOX, width_height, shadow)
            }
            Upgrade::Fang(..) => sticker_thumbnail(thumbnail_image::FANG, width_height, shadow),
            Upgrade::Popcorn(..) => {
                sticker_thumbnail(thumbnail_image::POPCORN, width_height, shadow)
            }
            Upgrade::MembershipCard(..) => {
                sticker_thumbnail(thumbnail_image::MEMBERSHIP_CARD, width_height, shadow)
            }
        }
    }
}
