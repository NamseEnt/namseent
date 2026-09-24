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
