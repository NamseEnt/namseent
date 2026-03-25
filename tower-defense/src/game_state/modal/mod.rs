mod settings;

#[cfg(feature = "debug-tools")]
use crate::game_state::debug_tools::DebugToolsModal;
use crate::game_state::modal::settings::SettingsModal;
use namui::*;

mod operation_plan;

use operation_plan::OperationPlanModal;

#[derive(State)]
pub enum Modal {
    Settings,
    OperationPlan,
    #[cfg(feature = "debug-tools")]
    DebugTools,
}

impl Component for &Modal {
    fn render(self, ctx: &RenderCtx) {
        match self {
            Modal::Settings => ctx.add(SettingsModal),
            Modal::OperationPlan => ctx.add(OperationPlanModal),
            #[cfg(feature = "debug-tools")]
            Modal::DebugTools => ctx.add(DebugToolsModal),
        };
    }
}
