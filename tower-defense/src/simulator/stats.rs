use anyhow::Context;
use rusqlite::{Connection, params};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

#[derive(Clone, Copy, Debug)]
pub struct SimulationOutcome {
    pub victory: bool,
    pub clear_rate: f64,
}

#[derive(Clone, Debug)]
pub struct SummaryRow {
    pub name: String,
    pub selected_simulations: usize,
    pub total_purchases: usize,
    pub win_rate: f64,
    pub avg_clear_rate: f64,
    pub clear_rate_variance: f64,
}

#[derive(Clone, Debug)]
pub struct StrategyStats {
    pub category: String,
    pub name: String,
    pub sample_count: usize,
    pub win_count: usize,
    pub win_rate: f64,
    pub avg_clear_rate: f64,
    pub clear_rate_variance: f64,
}

#[derive(Clone, Debug)]
pub struct PickBin {
    pub count: usize,
    pub sample_count: usize,
    pub win_count: usize,
    pub win_rate: f64,
}

#[derive(Clone, Debug)]
pub struct ClearRateBin {
    pub label: String,
    pub sample_count: usize,
    pub average_clear_rate: f64,
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
    pub avg_clear_rate: f64,
    pub clear_rate_variance: f64,
    pub clear_rate_samples: usize,
    pub clear_rate_distribution: Vec<ClearRateBin>,
    pub distribution: Vec<PickBin>,
}

const ITEM_NAMES: &[&str] = &[
    "RiceBall",
    "LumpSugar",
    "Shield",
    "Painkiller",
    "GrantBarricades",
    "GrantCard",
];

const UPGRADE_NAMES: &[&str] = &[
    "Cat",
    "Backpack",
    "DiceBundle",
    "EnergyDrink",
    "FourLeafClover",
    "Rabbit",
    "BlackWhite",
    "Eraser",
    "Staff",
    "LongSword",
    "Mace",
    "ClubSword",
    "Tricycle",
    "PerfectPottery",
    "SingleChopstick",
    "PairChopsticks",
    "FountainPen",
    "Brush",
    "BrokenPottery",
];

const STRATEGY_COLUMNS: &[(&str, &str)] = &[
    ("Shop", "shop_strategy"),
    ("Card Reroll", "card_reroll_strategy"),
    ("Tower Placement", "tower_placement_strategy"),
    ("Item Use", "item_use_strategy"),
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

    pub fn list_strategy_win_rates(&self) -> anyhow::Result<Vec<StrategyStats>> {
        let mut result = Vec::new();
        for (category, column) in STRATEGY_COLUMNS {
            let rows = self.query_strategy_stats(column)?;
            for row in rows {
                result.push(StrategyStats {
                    category: category.to_string(),
                    name: row.name,
                    sample_count: row.sample_count,
                    win_count: row.win_count,
                    win_rate: row.win_rate,
                    avg_clear_rate: row.avg_clear_rate,
                    clear_rate_variance: row.clear_rate_variance,
                });
            }
        }
        Ok(result)
    }

    fn query_strategy_stats(&self, column: &str) -> anyhow::Result<Vec<StrategyStats>> {
        let sql = format!(
            "SELECT {column}, COUNT(*) AS sample_count, SUM(COALESCE(victory, 0)) AS win_count, AVG(clear_rate) AS avg_clear_rate, AVG(clear_rate * clear_rate) AS avg_clear_rate_sq, AVG(CAST(COALESCE(victory, 0) AS REAL)) AS win_rate FROM simulations WHERE completed_at IS NOT NULL GROUP BY {column} ORDER BY avg_clear_rate DESC",
            column = column,
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let sample_count: i64 = row.get(1)?;
            let win_count: i64 = row.get(2)?;
            let avg_clear_rate: f64 = row.get(3)?;
            let avg_clear_rate_sq: f64 = row.get(4)?;
            let win_rate: f64 = row.get(5)?;
            let variance = (avg_clear_rate_sq - avg_clear_rate * avg_clear_rate).max(0.0);
            Ok(StrategyStats {
                category: String::new(),
                name,
                sample_count: sample_count as usize,
                win_count: win_count as usize,
                win_rate,
                avg_clear_rate,
                clear_rate_variance: variance,
            })
        })?;

        let mut output = Vec::new();
        for row in rows {
            output.push(row?);
        }
        Ok(output)
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

    fn load_simulation_outcomes(&self) -> anyhow::Result<HashMap<String, SimulationOutcome>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, victory, clear_rate FROM simulations WHERE completed_at IS NOT NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let victory: Option<i32> = row.get(1)?;
            let clear_rate: Option<f64> = row.get(2)?;
            Ok((
                id,
                SimulationOutcome {
                    victory: victory.unwrap_or(0) == 1,
                    clear_rate: clear_rate.unwrap_or(0.0),
                },
            ))
        })?;

        let mut outcome = HashMap::new();
        for row in rows {
            let (id, outcome_value) = row?;
            outcome.insert(id, outcome_value);
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
        outcome: &HashMap<String, SimulationOutcome>,
    ) -> Vec<SummaryRow> {
        let mut builder: HashMap<String, SummaryBuilder> = HashMap::new();

        for (simulation_id, kind) in rows {
            let entry = builder.entry(kind).or_default();
            entry.total_purchases += 1;
            if entry.simulation_ids.insert(simulation_id.clone())
                && let Some(outcome_value) = outcome.get(&simulation_id)
            {
                entry.clear_rate_samples += 1;
                entry.sum_clear_rate += outcome_value.clear_rate;
                entry.sum_clear_rate_sq += outcome_value.clear_rate * outcome_value.clear_rate;
            }
        }

        let mut result: Vec<SummaryRow> = builder
            .into_iter()
            .map(|(name, entry)| {
                let selected_simulations = entry.simulation_ids.len();
                let win_count = entry
                    .simulation_ids
                    .iter()
                    .filter(|id| outcome.get(*id).map(|o| o.victory).unwrap_or(false))
                    .count();
                let win_rate = if selected_simulations > 0 {
                    win_count as f64 / selected_simulations as f64
                } else {
                    0.0
                };
                let avg_clear_rate = if entry.clear_rate_samples > 0 {
                    entry.sum_clear_rate / entry.clear_rate_samples as f64
                } else {
                    0.0
                };
                let clear_rate_variance = if entry.clear_rate_samples > 0 {
                    (entry.sum_clear_rate_sq / entry.clear_rate_samples as f64)
                        - avg_clear_rate * avg_clear_rate
                } else {
                    0.0
                }
                .max(0.0);
                SummaryRow {
                    name,
                    selected_simulations,
                    total_purchases: entry.total_purchases,
                    win_rate,
                    avg_clear_rate,
                    clear_rate_variance,
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
                    avg_clear_rate: 0.0,
                    clear_rate_variance: 0.0,
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
            .filter(|id| outcome.get(*id).map(|o| o.victory).unwrap_or(false))
            .count();
        let win_rate = if selected_simulations > 0 {
            win_count as f64 / selected_simulations as f64
        } else {
            0.0
        };

        let mut distribution: BTreeMap<usize, PickBin> = BTreeMap::new();
        let mut clear_rate_values = Vec::new();
        for (simulation_id, count) in &counts {
            let bin = distribution.entry(*count).or_insert(PickBin {
                count: *count,
                sample_count: 0,
                win_count: 0,
                win_rate: 0.0,
            });
            bin.sample_count += 1;
            if let Some(outcome_value) = outcome.get(simulation_id) {
                if outcome_value.victory {
                    bin.win_count += 1;
                }
                clear_rate_values.push(outcome_value.clear_rate);
            }
        }

        let mut zero_pick_samples = 0;
        let mut zero_pick_win_count = 0;
        for (simulation_id, outcome_value) in &outcome {
            if !counts.contains_key(simulation_id) {
                zero_pick_samples += 1;
                if outcome_value.victory {
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

        let (avg_clear_rate, clear_rate_variance) = if !clear_rate_values.is_empty() {
            let sum: f64 = clear_rate_values.iter().copied().sum();
            let sum_sq: f64 = clear_rate_values.iter().copied().map(|v| v * v).sum();
            let count = clear_rate_values.len() as f64;
            let avg = sum / count;
            let variance = (sum_sq / count) - avg * avg;
            (avg, variance.max(0.0))
        } else {
            (0.0, 0.0)
        };

        let clear_rate_samples = clear_rate_values.len();
        let clear_rate_distribution = Self::build_clear_rate_distribution(&clear_rate_values);

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
            avg_clear_rate,
            clear_rate_variance,
            clear_rate_samples,
            clear_rate_distribution,
            distribution,
        })
    }

    fn build_clear_rate_distribution(clear_rates: &[f64]) -> Vec<ClearRateBin> {
        let mut bins: Vec<ClearRateBin> = (0..51)
            .map(|idx| ClearRateBin {
                label: format!("{:02}", idx + 1),
                sample_count: 0,
                average_clear_rate: 0.0,
            })
            .collect();

        for &rate in clear_rates {
            let bin_index = ((rate.clamp(0.0, 100.0) as usize) / 2).min(50);
            let bin = &mut bins[bin_index];
            bin.sample_count += 1;
            bin.average_clear_rate += rate;
        }

        bins.iter_mut()
            .map(|bin| {
                if bin.sample_count > 0 {
                    bin.average_clear_rate /= bin.sample_count as f64;
                }
                bin.clone()
            })
            .collect()
    }

    pub fn detail_for_strategy(
        &self,
        category: &str,
        strategy_name: &str,
    ) -> anyhow::Result<DetailStats> {
        let outcome = self.load_simulation_outcomes()?;
        let total_simulations = outcome.len();

        let column = match category {
            "Shop" => "shop_strategy",
            "Card Reroll" => "card_reroll_strategy",
            "Tower Placement" => "tower_placement_strategy",
            "Item Use" => "item_use_strategy",
            _ => return Err(anyhow::anyhow!("Unknown strategy category: {}", category)),
        };

        let sql = format!(
            "SELECT id, victory, clear_rate FROM simulations WHERE completed_at IS NOT NULL AND {column} = ?1",
            column = column,
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(params![strategy_name], |row| {
            let id: String = row.get(0)?;
            let victory: Option<i32> = row.get(1)?;
            let clear_rate: Option<f64> = row.get(2)?;
            Ok((
                id,
                SimulationOutcome {
                    victory: victory.unwrap_or(0) == 1,
                    clear_rate: clear_rate.unwrap_or(0.0),
                },
            ))
        })?;

        let mut selected_simulations = 0;
        let mut total_purchases = 0;
        let mut win_count = 0;
        let mut clear_rate_values = Vec::new();

        for row in rows {
            let (_id, strategy_outcome) = row?;
            selected_simulations += 1;
            total_purchases += 1;
            if strategy_outcome.victory {
                win_count += 1;
            }
            clear_rate_values.push(strategy_outcome.clear_rate);
        }

        let win_rate = if selected_simulations > 0 {
            win_count as f64 / selected_simulations as f64
        } else {
            0.0
        };

        let (avg_clear_rate, clear_rate_variance) = if !clear_rate_values.is_empty() {
            let sum: f64 = clear_rate_values.iter().copied().sum();
            let sum_sq: f64 = clear_rate_values.iter().copied().map(|v| v * v).sum();
            let count = clear_rate_values.len() as f64;
            let avg = sum / count;
            let variance = (sum_sq / count) - avg * avg;
            (avg, variance.max(0.0))
        } else {
            (0.0, 0.0)
        };

        let clear_rate_samples = clear_rate_values.len();
        let clear_rate_distribution = Self::build_clear_rate_distribution(&clear_rate_values);

        Ok(DetailStats {
            name: strategy_name.to_owned(),
            total_simulations,
            selected_simulations,
            total_purchases,
            win_rate,
            zero_pick_samples: 0,
            zero_pick_win_rate: 0.0,
            avg_clear_rate,
            clear_rate_variance,
            clear_rate_samples,
            clear_rate_distribution,
            distribution: Vec::new(),
        })
    }
}

#[derive(Default)]
struct SummaryBuilder {
    total_purchases: usize,
    clear_rate_samples: usize,
    sum_clear_rate: f64,
    sum_clear_rate_sq: f64,
    simulation_ids: HashSet<String>,
}
