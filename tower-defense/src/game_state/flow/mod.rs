use super::{GameState, monster_spawn::start_spawn, tower::TowerTemplate};
use crate::{card::Card, hand::HandItem, shop::Shop, sound, *};

#[cfg(feature = "debug-tools")]
fn save_stage_snapshot(game_state: &GameState) {
    crate::game_state::debug_tools::state_snapshot::save_snapshot_from_state(game_state);
}

#[cfg(not(feature = "debug-tools"))]
fn save_stage_snapshot(_game_state: &GameState) {}

#[derive(Clone, Debug, State)]
#[allow(clippy::large_enum_variant)]
pub enum GameFlow {
    Initializing,
    SelectingTower(SelectingTowerFlow),
    PlacingTower,
    Defense(DefenseFlow),
    TreasureSelection(TreasureSelectionFlow),
    Result { clear_rate: f32 },
}

#[derive(Clone, Debug, State)]
pub struct TreasureSelectionFlow {
    pub options: Vec<crate::game_state::upgrade::Upgrade>,
    pub pending_selection: Option<usize>,
}

impl TreasureSelectionFlow {
    pub fn new(game_state: &GameState) -> Self {
        let options = (0..3)
            .map(|_| crate::game_state::upgrade::generate_treasure_upgrade(game_state))
            .collect();
        TreasureSelectionFlow {
            options,
            pending_selection: None,
        }
    }

    fn update(&mut self) {}
}
impl GameFlow {
    pub(crate) fn update(&mut self) {
        match self {
            GameFlow::SelectingTower(selecting_tower) => selecting_tower.update(),
            GameFlow::TreasureSelection(treasure_flow) => treasure_flow.update(),
            _ => {}
        }
    }
}

#[derive(Clone, Debug, State)]
pub struct SelectingTowerFlow {
    pub shop: Shop,
}

impl SelectingTowerFlow {
    pub fn new(game_state: &GameState) -> Self {
        let shop = Shop::new(game_state);
        SelectingTowerFlow { shop }
    }

    fn update(&mut self) {
        self.shop.update();
    }
}

#[derive(Clone, Debug, State)]
pub struct StageProgress {
    pub start_total_hp: f32,
    pub processed_hp: f32,
}

#[derive(Clone, Debug, State)]
pub struct DefenseFlow {
    pub stage_progress: StageProgress,
}

impl DefenseFlow {
    pub fn new(game_state: &GameState) -> Self {
        let start_total_hp =
            GameState::calculate_stage_total_hp(game_state.stage, &game_state.stage_modifiers);
        Self {
            stage_progress: StageProgress {
                start_total_hp,
                processed_hp: 0.0,
            },
        }
    }
}

impl GameState {
    fn prepare_next_stage(&mut self) {
        self.stage_modifiers.reset_stage_state();
        self.left_dice = self.max_dice_chance();
        self.shield = 0.0;
        self.item_used = false;
        self.rerolled_count = 0;
        self.record_stage_start();
        save_stage_snapshot(self);
    }

    pub fn goto_next_stage(&mut self) {
        self.prepare_next_stage();
        self.goto_selecting_tower();
    }

    pub fn goto_selecting_tower(&mut self) {
        self.hand_panel_forced_open = true;
        self.shop_panel_forced_open = true;

        let max_slots = (5 + self.stage_modifiers.get_max_hand_slots_bonus())
            .saturating_sub(self.stage_modifiers.get_max_hand_slots_penalty())
            .max(1);
        sound::play_card_draw_sounds(max_slots);

        let removing_ids = self.hand.active_slot_ids();
        if !removing_ids.is_empty() {
            self.hand.delete_slots(&removing_ids);
        }
        for _ in 0..max_slots {
            self.hand.push(HandItem::Card(Card::new_random()));
        }

        self.flow = GameFlow::SelectingTower(SelectingTowerFlow::new(self));
    }

    pub fn goto_placing_tower(&mut self, tower_template: TowerTemplate) {
        let mut hand_items = vec![tower_template];

        // Drain all queued extra towers (barricades, special towers, etc.)
        for (tower_kind, suit, rank) in self.stage_modifiers.drain_extra_tower_cards() {
            hand_items.push(TowerTemplate::new(tower_kind, suit, rank));
        }

        let removing_ids = self.hand.active_slot_ids();
        if !removing_ids.is_empty() {
            self.hand.delete_slots(&removing_ids);
        }

        for tower in hand_items {
            self.hand.push(HandItem::Tower(tower));
        }

        if let Some(first_slot_id) = self.hand.get_slot_id_by_index(0)
            && self
                .hand
                .get_item(first_slot_id)
                .and_then(|item| item.as_tower())
                .is_some()
        {
            self.hand.select_slot(first_slot_id);
        }

        self.flow = GameFlow::PlacingTower;
    }

    pub fn goto_defense(&mut self) {
        self.flow = GameFlow::Defense(DefenseFlow::new(self));
        sound::emit_sound(
            sound::EmitSoundParams::one_shot(
                sound::random_trumpet_fanfares(),
                sound::SoundGroup::Ui,
                sound::VolumePreset::High,
                sound::SpatialMode::NonSpatial,
            )
            .with_max_duration(Duration::from_secs(6)),
        );
        start_spawn(self);
    }

    pub fn goto_treasure_selection(&mut self) {
        self.flow = GameFlow::TreasureSelection(TreasureSelectionFlow::new(self));
    }

    pub fn select_treasure(&mut self, index: usize) {
        if let GameFlow::TreasureSelection(flow) = &self.flow
            && index < flow.options.len()
        {
            let upgrade = flow.options[index];
            self.upgrade_state.upgrade(upgrade);
        }
        self.goto_next_stage();
    }

    pub fn goto_result(&mut self) {
        let clear_rate = self.calculate_clear_rate();

        // 남은 모든 적 제거 (패배 후에도 적들이 건물에 들어오는 것을 방지)
        self.monsters.clear();
        self.projectiles.clear();
        self.record_game_over();
        self.flow = GameFlow::Result { clear_rate };
    }
}
