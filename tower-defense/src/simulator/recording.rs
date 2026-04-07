//! SQLite recording for simulation results.

use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Mutex;

use super::events::SimEvent;

pub struct SimRecorder {
    conn: Mutex<Connection>,
}

impl SimRecorder {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;
        let recorder = Self {
            conn: Mutex::new(conn),
        };
        recorder.create_tables()?;
        Ok(recorder)
    }

    fn create_tables(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS simulations (
                id TEXT PRIMARY KEY,
                started_at TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at TEXT,
                shop_strategy TEXT NOT NULL,
                card_reroll_strategy TEXT NOT NULL,
                tower_placement_strategy TEXT NOT NULL,
                item_use_strategy TEXT NOT NULL,
                seed INTEGER NOT NULL,
                victory INTEGER,
                final_stage INTEGER,
                clear_rate REAL,
                final_hp REAL,
                final_gold INTEGER,
                total_towers_placed INTEGER DEFAULT 0,
                total_items_used INTEGER DEFAULT 0,
                total_damage_taken REAL DEFAULT 0,
                total_gold_earned INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS simulation_upgrades (
                simulation_id TEXT NOT NULL,
                upgrade_kind TEXT NOT NULL,
                stage_acquired INTEGER NOT NULL,
                damage_multiplier REAL,
                FOREIGN KEY (simulation_id) REFERENCES simulations(id)
            );

            CREATE TABLE IF NOT EXISTS simulation_items (
                simulation_id TEXT NOT NULL,
                item_kind TEXT NOT NULL,
                stage_used INTEGER,
                times_used INTEGER DEFAULT 0,
                FOREIGN KEY (simulation_id) REFERENCES simulations(id)
            );

            CREATE TABLE IF NOT EXISTS simulation_stage_results (
                simulation_id TEXT NOT NULL,
                stage INTEGER NOT NULL,
                victory INTEGER NOT NULL,
                hp_before REAL,
                hp_after REAL,
                towers_placed INTEGER DEFAULT 0,
                gold_before INTEGER,
                gold_after INTEGER,
                tower_kind_selected TEXT,
                rerolls_used INTEGER DEFAULT 0,
                PRIMARY KEY (simulation_id, stage),
                FOREIGN KEY (simulation_id) REFERENCES simulations(id)
            );

            CREATE TABLE IF NOT EXISTS simulation_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                simulation_id TEXT NOT NULL,
                event_order INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                event_data TEXT NOT NULL,
                FOREIGN KEY (simulation_id) REFERENCES simulations(id)
            );

            CREATE INDEX IF NOT EXISTS idx_sim_victory ON simulations(victory);
            CREATE INDEX IF NOT EXISTS idx_sim_strategy ON simulations(shop_strategy, card_reroll_strategy, tower_placement_strategy, item_use_strategy);
            CREATE INDEX IF NOT EXISTS idx_sim_stage ON simulation_stage_results(simulation_id, stage);
            CREATE INDEX IF NOT EXISTS idx_sim_upgrades ON simulation_upgrades(simulation_id);
            CREATE INDEX IF NOT EXISTS idx_sim_events ON simulation_events(simulation_id);
            ",
        )?;
        Ok(())
    }

    pub fn record_simulation_start(
        &self,
        sim_id: &str,
        shop_strategy: &str,
        card_reroll_strategy: &str,
        tower_placement_strategy: &str,
        item_use_strategy: &str,
        seed: u64,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO simulations (id, shop_strategy, card_reroll_strategy, tower_placement_strategy, item_use_strategy, seed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![sim_id, shop_strategy, card_reroll_strategy, tower_placement_strategy, item_use_strategy, seed as i64],
        )?;
        Ok(())
    }

    pub fn record_simulation_end(
        &self,
        sim_id: &str,
        victory: bool,
        final_stage: usize,
        clear_rate: f32,
        final_hp: f32,
        final_gold: usize,
        total_towers_placed: usize,
        total_items_used: usize,
        total_damage_taken: f32,
        total_gold_earned: usize,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE simulations SET completed_at = datetime('now'), victory = ?2, final_stage = ?3, clear_rate = ?4, final_hp = ?5, final_gold = ?6, total_towers_placed = ?7, total_items_used = ?8, total_damage_taken = ?9, total_gold_earned = ?10 WHERE id = ?1",
            params![sim_id, victory as i32, final_stage as i64, clear_rate as f64, final_hp as f64, final_gold as i64, total_towers_placed as i64, total_items_used as i64, total_damage_taken as f64, total_gold_earned as i64],
        )?;
        Ok(())
    }

    pub fn record_stage_result(
        &self,
        sim_id: &str,
        stage: usize,
        victory: bool,
        hp_before: f32,
        hp_after: f32,
        towers_placed: usize,
        gold_before: usize,
        gold_after: usize,
        tower_kind: &str,
        rerolls_used: usize,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO simulation_stage_results (simulation_id, stage, victory, hp_before, hp_after, towers_placed, gold_before, gold_after, tower_kind_selected, rerolls_used) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![sim_id, stage as i64, victory as i32, hp_before as f64, hp_after as f64, towers_placed as i64, gold_before as i64, gold_after as i64, tower_kind, rerolls_used as i64],
        )?;
        Ok(())
    }

    pub fn record_upgrade(
        &self,
        sim_id: &str,
        upgrade_kind: &str,
        stage: usize,
        damage_multiplier: Option<f32>,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO simulation_upgrades (simulation_id, upgrade_kind, stage_acquired, damage_multiplier) VALUES (?1, ?2, ?3, ?4)",
            params![sim_id, upgrade_kind, stage as i64, damage_multiplier.map(|m| m as f64)],
        )?;
        Ok(())
    }

    pub fn record_events(
        &self,
        sim_id: &str,
        events: &[SimEvent],
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "INSERT INTO simulation_events (simulation_id, event_order, event_type, event_data) VALUES (?1, ?2, ?3, ?4)",
        )?;

        for (i, event) in events.iter().enumerate() {
            let event_type = match event {
                SimEvent::GameStart => "game_start",
                SimEvent::StageStart { .. } => "stage_start",
                SimEvent::ShopReroll { .. } => "shop_reroll",
                SimEvent::ShopPurchase { .. } => "shop_purchase",
                SimEvent::CardReroll { .. } => "card_reroll",
                SimEvent::TowerSelected { .. } => "tower_selected",
                SimEvent::TowerPlaced { .. } => "tower_placed",
                SimEvent::TowerRemoved { .. } => "tower_removed",
                SimEvent::DefenseStart { .. } => "defense_start",
                SimEvent::DefenseEnd { .. } => "defense_end",
                SimEvent::DamageTaken { .. } => "damage_taken",
                SimEvent::MonsterKilled { .. } => "monster_killed",
                SimEvent::ItemUsed { .. } => "item_used",
                SimEvent::TreasureSelected { .. } => "treasure_selected",
                SimEvent::GameEnd { .. } => "game_end",
            };
            let event_data = serde_json::to_string(event)?;
            stmt.execute(params![sim_id, i as i64, event_type, event_data])?;
        }

        Ok(())
    }
}
