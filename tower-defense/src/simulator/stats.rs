use anyhow::Context;
use rusqlite::{Connection, params};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

#[derive(Clone, Debug)]
pub struct SummaryRow {
    pub name: String,
    pub selected_simulations: usize,
    pub total_purchases: usize,
    pub win_rate: f64,
}

#[derive(Clone, Debug)]
pub struct PickBin {
    pub count: usize,
    pub sample_count: usize,
    pub win_count: usize,
    pub win_rate: f64,
}

#[derive(Clone, Debug)]
pub struct DetailStats {
    pub name: String,
    pub total_simulations: usize,
    pub selected_simulations: usize,
    pub total_purchases: usize,
    pub win_rate: f64,
    pub zero_pick_samples: usize,
    pub zero_pick_win_rate: f64,
    pub distribution: Vec<PickBin>,
}

const ITEM_NAMES: &[&str] = &[
    "RiceCake",
    "EmergencyDice",
    "Shield",
    "Painkiller",
    "GrantBarricades",
    "GrantCard",
];

const UPGRADE_NAMES: &[&str] = &[
    "Magnet",
    "Backpack",
    "DiceBundle",
    "EnergyDrink",
    "FourLeafClover",
    "Rabbit",
    "BlackWhite",
    "Eraser",
    "CainSword",
    "LongSword",
    "Mace",
    "ClubSword",
    "Spoon",
    "PerfectPottery",
    "SingleChopstick",
    "PairChopsticks",
    "FountainPen",
    "Brush",
    "BrokenPottery",
];

impl Database {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database: {}", path.display()))?;
        Ok(Self { conn })
    }

    pub fn list_items(&self) -> anyhow::Result<Vec<SummaryRow>> {
        let summary = self.list_shop_purchase_summaries()?;
        Ok(self.build_known_summary(summary, ITEM_NAMES))
    }

    pub fn list_upgrades(&self) -> anyhow::Result<Vec<SummaryRow>> {
        let summary = self.list_shop_purchase_summaries()?;
        Ok(self.build_known_summary(summary, UPGRADE_NAMES))
    }

    pub fn list_treasures(&self) -> anyhow::Result<Vec<SummaryRow>> {
        let outcome = self.load_simulation_outcomes()?;
        let rows = self.query_event_kind("treasure_selected", "$.TreasureSelected.upgrade_kind")?;
        Ok(self.build_summary(rows, &outcome))
    }

    pub fn detail_for_shop_purchase(&self, kind: &str) -> anyhow::Result<DetailStats> {
        self.detail_for_event("shop_purchase", "$.ShopPurchase.item_kind", kind)
    }

    pub fn detail_for_treasure(&self, kind: &str) -> anyhow::Result<DetailStats> {
        self.detail_for_event("treasure_selected", "$.TreasureSelected.upgrade_kind", kind)
    }

    fn list_shop_purchase_summaries(&self) -> anyhow::Result<Vec<SummaryRow>> {
        let outcome = self.load_simulation_outcomes()?;
        let rows = self.query_event_kind("shop_purchase", "$.ShopPurchase.item_kind")?;
        Ok(self.build_summary(rows, &outcome))
    }

    fn load_simulation_outcomes(&self) -> anyhow::Result<HashMap<String, bool>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, victory FROM simulations WHERE completed_at IS NOT NULL")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let victory: Option<i32> = row.get(1)?;
            Ok((id, victory.unwrap_or(0) == 1))
        })?;

        let mut outcome = HashMap::new();
        for row in rows {
            let (id, victory) = row?;
            outcome.insert(id, victory);
        }
        Ok(outcome)
    }

    fn query_event_kind(
        &self,
        event_type: &str,
        json_path: &str,
    ) -> anyhow::Result<Vec<(String, String)>> {
        let sql = format!(
            "SELECT simulation_id, json_extract(event_data, '{}') FROM simulation_events WHERE event_type = ?1",
            json_path,
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(params![event_type], |row| {
            let simulation_id: String = row.get(0)?;
            let kind: Option<String> = row.get(1)?;
            Ok((simulation_id, kind.unwrap_or_default()))
        })?;

        let mut output = Vec::new();
        for row in rows {
            let (simulation_id, kind) = row?;
            let kind = Self::canonicalize_kind_name(kind);
            if kind.is_empty() {
                continue;
            }
            output.push((simulation_id, kind));
        }
        Ok(output)
    }

    fn canonicalize_kind_name(kind: String) -> String {
        kind.split_whitespace().next().unwrap_or(&kind).to_string()
    }

    fn build_summary(
        &self,
        rows: Vec<(String, String)>,
        outcome: &HashMap<String, bool>,
    ) -> Vec<SummaryRow> {
        let mut builder: HashMap<String, SummaryBuilder> = HashMap::new();

        for (simulation_id, kind) in rows {
            let entry = builder.entry(kind).or_default();
            entry.total_purchases += 1;
            entry.simulation_ids.insert(simulation_id);
        }

        let mut result: Vec<SummaryRow> = builder
            .into_iter()
            .map(|(name, entry)| {
                let selected_simulations = entry.simulation_ids.len();
                let win_count = entry
                    .simulation_ids
                    .iter()
                    .filter(|id| outcome.get(*id).copied().unwrap_or(false))
                    .count();
                let win_rate = if selected_simulations > 0 {
                    win_count as f64 / selected_simulations as f64
                } else {
                    0.0
                };
                SummaryRow {
                    name,
                    selected_simulations,
                    total_purchases: entry.total_purchases,
                    win_rate,
                }
            })
            .collect();

        result.sort_by(|a, b| {
            b.win_rate
                .partial_cmp(&a.win_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        result
    }

    fn build_known_summary(&self, rows: Vec<SummaryRow>, known_names: &[&str]) -> Vec<SummaryRow> {
        let mut map: HashMap<String, SummaryRow> = HashMap::new();
        for row in rows {
            map.insert(row.name.clone(), row);
        }

        let mut result: Vec<SummaryRow> = known_names
            .iter()
            .map(|name| {
                map.remove(*name).unwrap_or(SummaryRow {
                    name: name.to_string(),
                    selected_simulations: 0,
                    total_purchases: 0,
                    win_rate: 0.0,
                })
            })
            .collect();

        result.sort_by(|a, b| {
            b.win_rate
                .partial_cmp(&a.win_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        result
    }

    fn detail_for_event(
        &self,
        event_type: &str,
        json_path: &str,
        kind: &str,
    ) -> anyhow::Result<DetailStats> {
        let outcome = self.load_simulation_outcomes()?;
        let total_simulations = outcome.len();
        let rows = self.query_event_kind(event_type, json_path)?;

        let mut counts: HashMap<String, usize> = HashMap::new();
        for (simulation_id, row_kind) in rows {
            if row_kind != kind {
                continue;
            }
            *counts.entry(simulation_id).or_default() += 1;
        }

        let selected_simulations = counts.len();
        let total_purchases: usize = counts.values().sum();
        let win_count = counts
            .keys()
            .filter(|id| outcome.get(*id).copied().unwrap_or(false))
            .count();
        let win_rate = if selected_simulations > 0 {
            win_count as f64 / selected_simulations as f64
        } else {
            0.0
        };

        let mut distribution: BTreeMap<usize, PickBin> = BTreeMap::new();
        for (simulation_id, count) in &counts {
            let bin = distribution.entry(*count).or_insert(PickBin {
                count: *count,
                sample_count: 0,
                win_count: 0,
                win_rate: 0.0,
            });
            bin.sample_count += 1;
            if outcome.get(simulation_id).copied().unwrap_or(false) {
                bin.win_count += 1;
            }
        }

        let mut zero_pick_samples = 0;
        let mut zero_pick_win_count = 0;
        for (simulation_id, victory) in &outcome {
            if !counts.contains_key(simulation_id) {
                zero_pick_samples += 1;
                if *victory {
                    zero_pick_win_count += 1;
                }
            }
        }

        let mut distribution: Vec<PickBin> = distribution.into_values().collect();
        for bin in &mut distribution {
            bin.win_rate = if bin.sample_count > 0 {
                bin.win_count as f64 / bin.sample_count as f64
            } else {
                0.0
            };
        }

        Ok(DetailStats {
            name: kind.to_owned(),
            total_simulations,
            selected_simulations,
            total_purchases,
            win_rate,
            zero_pick_samples,
            zero_pick_win_rate: if zero_pick_samples > 0 {
                zero_pick_win_count as f64 / zero_pick_samples as f64
            } else {
                0.0
            },
            distribution,
        })
    }
}

#[derive(Default)]
struct SummaryBuilder {
    total_purchases: usize,
    simulation_ids: HashSet<String>,
}
