use namui::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct TowerInfoSpringState {
    pub scale: f32,
    pub scale_velocity: f32,
    pub opacity: f32,
    pub opacity_velocity: f32,
    pub target_scale: f32,
    pub target_opacity: f32,
    pub last_tick: Instant,
}

impl TowerInfoSpringState {
    pub fn new(now: Instant) -> Self {
        Self {
            scale: 0.0,
            scale_velocity: 0.0,
            opacity: 0.0,
            opacity_velocity: 0.0,
            target_scale: 0.0,
            target_opacity: 0.0,
            last_tick: now,
        }
    }

    pub fn show(&mut self) {
        self.target_scale = 1.0;
        self.target_opacity = 1.0;
    }

    pub fn hide(&mut self) {
        self.target_scale = 0.0;
        self.target_opacity = 0.0;
    }

    pub fn tick(&mut self, now: Instant) {
        const STIFFNESS: f32 = 300.0;
        const DAMPING: f32 = 20.0;

        let delta_time = (now - self.last_tick).as_secs_f32().min(0.016); // Cap at 60fps
        self.last_tick = now;

        // Scale spring
        let scale_force = STIFFNESS * (self.target_scale - self.scale);
        let scale_damping = -DAMPING * self.scale_velocity;
        let scale_acceleration = scale_force + scale_damping;
        self.scale_velocity += scale_acceleration * delta_time;
        self.scale += self.scale_velocity * delta_time;

        // Opacity spring
        let opacity_force = STIFFNESS * (self.target_opacity - self.opacity);
        let opacity_damping = -DAMPING * self.opacity_velocity;
        let opacity_acceleration = opacity_force + opacity_damping;
        self.opacity_velocity += opacity_acceleration * delta_time;
        self.opacity += self.opacity_velocity * delta_time;

        // Clamp values
        self.scale = self.scale.max(0.0);
        self.opacity = self.opacity.clamp(0.0, 1.0);
    }

    pub fn is_visible(&self) -> bool {
        self.opacity > 0.01 || self.target_opacity > 0.0
    }
}

/// UI 관련 상태를 관리하는 별도 구조체
pub struct UIState {
    pub tower_popup_states: HashMap<usize, TowerInfoSpringState>,
    pub selected_tower_id: Option<usize>,
    last_cleanup_time: Instant,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            tower_popup_states: HashMap::new(),
            selected_tower_id: None,
            last_cleanup_time: Instant::now(),
        }
    }

    pub fn ensure_tower_popup_state(&mut self, tower_id: usize, now: Instant) {
        self.tower_popup_states
            .entry(tower_id)
            .or_insert_with(|| TowerInfoSpringState::new(now));
    }

    pub fn set_selected_tower(&mut self, tower_id: Option<usize>, now: Instant) {
        // Early return if same tower
        if self.selected_tower_id == tower_id {
            return;
        }

        // Hide previously selected tower
        if let Some(prev_id) = self.selected_tower_id
            && let Some(popup_state) = self.tower_popup_states.get_mut(&prev_id)
        {
            popup_state.hide();
        }

        // Show newly selected tower
        if let Some(new_id) = tower_id {
            self.ensure_tower_popup_state(new_id, now);
            if let Some(popup_state) = self.tower_popup_states.get_mut(&new_id) {
                popup_state.show();
            }
        }

        self.selected_tower_id = tower_id;
    }

    pub fn tick(&mut self, now: Instant) {
    // Update all popup spring states
        for popup_state in self.tower_popup_states.values_mut() {
            popup_state.tick(now);
        }

        // Cleanup unused states periodically (every 5 seconds)
        if (now - self.last_cleanup_time) > Duration::from_secs(5) {
            self.last_cleanup_time = now;
            // Note: cleanup will be called externally with tower list
        }
    }

    pub fn cleanup_unused_states(&mut self, existing_tower_ids: &std::collections::HashSet<usize>) {
        self.tower_popup_states
            .retain(|&tower_id, _| existing_tower_ids.contains(&tower_id));

        // Also clear selected_tower_id if the tower no longer exists
        if let Some(selected_id) = self.selected_tower_id
            && !existing_tower_ids.contains(&selected_id)
        {
            self.selected_tower_id = None;
        }
    }

    pub fn get_popup_state(&self, tower_id: usize) -> Option<&TowerInfoSpringState> {
        self.tower_popup_states.get(&tower_id)
    }

    pub fn should_cleanup(&self, now: Instant) -> bool {
        (now - self.last_cleanup_time) > Duration::from_secs(5)
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}
