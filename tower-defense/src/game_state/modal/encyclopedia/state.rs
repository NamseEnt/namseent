pub(crate) use crate::game_state::discovery::DiscoveryState;

pub async fn load_async() -> DiscoveryState {
    crate::game_state::discovery::load_async().await
}
