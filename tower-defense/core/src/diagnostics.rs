use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct Counters {
    pub can_place_at_calls: u64,
    pub can_place_at_nanos: u64,
    pub placement_bfs_calls: u64,
    pub shortest_route_bfs_calls: u64,
    pub semantic_legal_positions_scans: u64,
    pub tower_placement_action_scans: u64,
    pub full_map_legality_mask_scans: u64,
    pub build_tower_legality_checks: u64,
    pub snapshot_calls: u64,
    pub rollout_decisions: u64,
}

impl Counters {
    pub fn delta(&self, earlier: &Counters) -> Counters {
        Counters {
            can_place_at_calls: self.can_place_at_calls - earlier.can_place_at_calls,
            can_place_at_nanos: self.can_place_at_nanos - earlier.can_place_at_nanos,
            placement_bfs_calls: self.placement_bfs_calls - earlier.placement_bfs_calls,
            shortest_route_bfs_calls: self.shortest_route_bfs_calls
                - earlier.shortest_route_bfs_calls,
            semantic_legal_positions_scans: self.semantic_legal_positions_scans
                - earlier.semantic_legal_positions_scans,
            tower_placement_action_scans: self.tower_placement_action_scans
                - earlier.tower_placement_action_scans,
            full_map_legality_mask_scans: self.full_map_legality_mask_scans
                - earlier.full_map_legality_mask_scans,
            build_tower_legality_checks: self.build_tower_legality_checks
                - earlier.build_tower_legality_checks,
            snapshot_calls: self.snapshot_calls - earlier.snapshot_calls,
            rollout_decisions: self.rollout_decisions - earlier.rollout_decisions,
        }
    }

    pub fn full_placement_scans(&self) -> u64 {
        self.semantic_legal_positions_scans
            + self.tower_placement_action_scans
            + self.full_map_legality_mask_scans
    }
}

thread_local! {
    static COUNTERS: Cell<Counters> = Cell::new(Counters::default());
}

pub fn record(update: impl FnOnce(&mut Counters)) {
    COUNTERS.with(|cell| {
        let mut counters = cell.get();
        update(&mut counters);
        cell.set(counters);
    });
}

pub fn snapshot() -> Counters {
    COUNTERS.with(Cell::get)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum Scope {
    SemanticStep,
    SemanticActionIsLegal,
    StepLegalityCheck,
    Snapshot,
    StateHash,
    RewardShaping,
    TraceConstruction,
    LegalActions,
    SemanticLegalActions,
    PlacementScan,
    DenseBuildTable,
    CanonicalPolicy,
    AdvanceUntilDecision,
    CoreTick,
    ApplyAction,
    Fork,
}

pub const SCOPES: [Scope; 16] = [
    Scope::SemanticStep,
    Scope::SemanticActionIsLegal,
    Scope::StepLegalityCheck,
    Scope::Snapshot,
    Scope::StateHash,
    Scope::RewardShaping,
    Scope::TraceConstruction,
    Scope::LegalActions,
    Scope::SemanticLegalActions,
    Scope::PlacementScan,
    Scope::DenseBuildTable,
    Scope::CanonicalPolicy,
    Scope::AdvanceUntilDecision,
    Scope::CoreTick,
    Scope::ApplyAction,
    Scope::Fork,
];

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct ScopeTotals {
    pub calls: u64,
    pub nanos: u64,
}

thread_local! {
    static SCOPE_TOTALS: Cell<[ScopeTotals; SCOPES.len()]> =
        Cell::new([ScopeTotals::default(); SCOPES.len()]);
}

pub struct ScopeGuard {
    scope: Scope,
    started: std::time::Instant,
}

impl ScopeGuard {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            started: std::time::Instant::now(),
        }
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        let nanos = self.started.elapsed().as_nanos() as u64;
        SCOPE_TOTALS.with(|cell| {
            let mut totals = cell.get();
            let entry = &mut totals[self.scope as usize];
            entry.calls += 1;
            entry.nanos += nanos;
            cell.set(totals);
        });
    }
}

pub fn scope_totals() -> [ScopeTotals; SCOPES.len()] {
    SCOPE_TOTALS.with(Cell::get)
}

pub fn scope_delta(
    later: &[ScopeTotals; SCOPES.len()],
    earlier: &[ScopeTotals; SCOPES.len()],
) -> Vec<(Scope, ScopeTotals)> {
    SCOPES
        .iter()
        .enumerate()
        .map(|(index, &scope)| {
            (
                scope,
                ScopeTotals {
                    calls: later[index].calls - earlier[index].calls,
                    nanos: later[index].nanos - earlier[index].nanos,
                },
            )
        })
        .collect()
}

