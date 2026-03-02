use namui::AudioAsset;
use rand::Rng;

const BUBBLE_POP_ASSETS: [AudioAsset; 14] = [
    crate::asset::sound::bubble_pop::BUBBLE_POP_00,
    crate::asset::sound::bubble_pop::BUBBLE_POP_01,
    crate::asset::sound::bubble_pop::BUBBLE_POP_02,
    crate::asset::sound::bubble_pop::BUBBLE_POP_03,
    crate::asset::sound::bubble_pop::BUBBLE_POP_04,
    crate::asset::sound::bubble_pop::BUBBLE_POP_05,
    crate::asset::sound::bubble_pop::BUBBLE_POP_06,
    crate::asset::sound::bubble_pop::BUBBLE_POP_07,
    crate::asset::sound::bubble_pop::BUBBLE_POP_08,
    crate::asset::sound::bubble_pop::BUBBLE_POP_09,
    crate::asset::sound::bubble_pop::BUBBLE_POP_10,
    crate::asset::sound::bubble_pop::BUBBLE_POP_11,
    crate::asset::sound::bubble_pop::BUBBLE_POP_12,
    crate::asset::sound::bubble_pop::BUBBLE_POP_13,
];

const MURCHUNGA_ASSETS: [AudioAsset; 9] = [
    crate::asset::sound::murchunga::MURCHUNGA_00,
    crate::asset::sound::murchunga::MURCHUNGA_01,
    crate::asset::sound::murchunga::MURCHUNGA_02,
    crate::asset::sound::murchunga::MURCHUNGA_03,
    crate::asset::sound::murchunga::MURCHUNGA_04,
    crate::asset::sound::murchunga::MURCHUNGA_05,
    crate::asset::sound::murchunga::MURCHUNGA_06,
    crate::asset::sound::murchunga::MURCHUNGA_07,
    crate::asset::sound::murchunga::MURCHUNGA_08,
];

const PICK_UP_CARDS_ASSETS: [AudioAsset; 6] = [
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_00,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_01,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_02,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_03,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_04,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_05,
];
const PICKAXE_ASSETS: [AudioAsset; 3] = [
    crate::asset::sound::pickaxe::PICKAXE_00,
    crate::asset::sound::pickaxe::PICKAXE_01,
    crate::asset::sound::pickaxe::PICKAXE_02,
];
const CLOTH_FOOTSTEP_ASSETS: [AudioAsset; 9] = [
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_00,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_01,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_02,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_03,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_04,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_05,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_06,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_07,
    crate::asset::sound::cloth_footstep::CLOTH_FOOTSTEP_08,
];
const PAPER_CRUMPLING_ASSETS: [AudioAsset; 6] = [
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_00,
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_01,
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_02,
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_03,
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_04,
    crate::asset::sound::paper_crumpling::PAPER_CRUMPLING_05,
];
const LUGGAGE_DROP_ASSETS: [AudioAsset; 3] = [
    crate::asset::sound::luggage_drop::LUGGAGE_DROP_00,
    crate::asset::sound::luggage_drop::LUGGAGE_DROP_01,
    crate::asset::sound::luggage_drop::LUGGAGE_DROP_02,
];
const WHOOSH_ASSETS: [AudioAsset; 5] = [
    crate::asset::sound::whoosh::WHOOSH_00,
    crate::asset::sound::whoosh::WHOOSH_01,
    crate::asset::sound::whoosh::WHOOSH_02,
    crate::asset::sound::whoosh::WHOOSH_03,
    crate::asset::sound::whoosh::WHOOSH_04,
];
const CRACKLING_FIRE_ASSETS: [AudioAsset; 1] =
    [crate::asset::sound::crackling_fire::CRACKLING_FIRE_00];
const FLAMETHROWER_ASSETS: [AudioAsset; 1] = [crate::asset::sound::flamethrower::FLAMETHROWER_00];
const RED_LASER_SHOT_ASSETS: [AudioAsset; 5] = [
    crate::asset::sound::red_laser_shot::RED_LASER_SHOT_00,
    crate::asset::sound::red_laser_shot::RED_LASER_SHOT_01,
    crate::asset::sound::red_laser_shot::RED_LASER_SHOT_02,
    crate::asset::sound::red_laser_shot::RED_LASER_SHOT_03,
    crate::asset::sound::red_laser_shot::RED_LASER_SHOT_04,
];
const SHINING_RINGING_ASSETS: [AudioAsset; 1] =
    [crate::asset::sound::shining_ringing::SHINING_RINGING_00];
const SMOKE_BOMB_ASSETS: [AudioAsset; 1] = [crate::asset::sound::smoke_bomb::SMOKE_BOMB_00];
const WIND_ASSETS: [AudioAsset; 1] = [crate::asset::sound::wind::WIND_00];
const KNIFE_SLASH_ASSETS: [AudioAsset; 4] = [
    crate::asset::sound::knife_slash::KNIFE_SLASH_00,
    crate::asset::sound::knife_slash::KNIFE_SLASH_01,
    crate::asset::sound::knife_slash::KNIFE_SLASH_02,
    crate::asset::sound::knife_slash::KNIFE_SLASH_03,
];

const LEVEL_UP_ASSETS: [AudioAsset; 1] = [crate::asset::sound::level_up::LEVEL_UP_00];
const ORCH_HIT_ASSETS: [AudioAsset; 1] = [crate::asset::sound::orch_hit::ORCH_HIT_00];
const FAIL_ASSETS: [AudioAsset; 1] = [crate::asset::sound::fail::FAIL_00];
const TRUMPET_FANFARES_CLEAN_ASSETS: [AudioAsset; 4] = [
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_00,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_01,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_02,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_03,
];
const TRUMPET_FANFARES_SILLY_ASSETS: [AudioAsset; 6] = [
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_04,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_05,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_06,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_07,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_08,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_09,
];
const TRUMPET_FANFARES_SILLY_PROBABILITY: f64 = 0.1;
const COIN_SOUNDS_ASSETS: [AudioAsset; 40] = [
    crate::asset::sound::coin_sounds::COIN_SOUNDS_00,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_01,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_02,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_03,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_04,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_05,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_06,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_07,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_08,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_09,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_10,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_11,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_12,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_13,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_14,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_15,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_16,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_17,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_18,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_19,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_20,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_21,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_22,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_23,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_24,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_25,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_26,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_27,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_28,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_29,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_30,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_31,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_32,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_33,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_34,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_35,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_36,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_37,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_38,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_39,
];

pub fn random_bubble_pop() -> AudioAsset {
    random_one(&BUBBLE_POP_ASSETS)
}

pub fn random_murchunga() -> AudioAsset {
    random_one(&MURCHUNGA_ASSETS)
}

pub fn random_pick_up_cards() -> AudioAsset {
    random_one(&PICK_UP_CARDS_ASSETS)
}

pub fn random_pickaxe() -> AudioAsset {
    random_one(&PICKAXE_ASSETS)
}

pub fn random_cloth_footstep() -> AudioAsset {
    random_one(&CLOTH_FOOTSTEP_ASSETS)
}

pub fn random_paper_crumpling() -> AudioAsset {
    random_one(&PAPER_CRUMPLING_ASSETS)
}

pub fn random_luggage_drop() -> AudioAsset {
    random_one(&LUGGAGE_DROP_ASSETS)
}

pub fn random_whoosh() -> AudioAsset {
    random_one(&WHOOSH_ASSETS)
}

pub fn random_crackling_fire() -> AudioAsset {
    random_one(&CRACKLING_FIRE_ASSETS)
}

pub fn random_flamethrower() -> AudioAsset {
    random_one(&FLAMETHROWER_ASSETS)
}

pub fn random_red_laser_shot() -> AudioAsset {
    random_one(&RED_LASER_SHOT_ASSETS)
}

pub fn random_shining_ringing() -> AudioAsset {
    random_one(&SHINING_RINGING_ASSETS)
}

pub fn random_smoke_bomb() -> AudioAsset {
    random_one(&SMOKE_BOMB_ASSETS)
}

pub fn random_wind() -> AudioAsset {
    random_one(&WIND_ASSETS)
}

pub fn random_knife_slash() -> AudioAsset {
    random_one(&KNIFE_SLASH_ASSETS)
}

pub fn random_level_up() -> AudioAsset {
    random_one(&LEVEL_UP_ASSETS)
}

pub fn random_orch_hit() -> AudioAsset {
    random_one(&ORCH_HIT_ASSETS)
}

pub fn random_fail() -> AudioAsset {
    random_one(&FAIL_ASSETS)
}

pub fn random_trumpet_fanfares() -> AudioAsset {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(TRUMPET_FANFARES_SILLY_PROBABILITY) {
        random_one(&TRUMPET_FANFARES_SILLY_ASSETS)
    } else {
        random_one(&TRUMPET_FANFARES_CLEAN_ASSETS)
    }
}

pub fn random_coin_sounds() -> AudioAsset {
    random_one(&COIN_SOUNDS_ASSETS)
}

fn random_one(assets: &[AudioAsset]) -> AudioAsset {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..assets.len());
    assets[index]
}
