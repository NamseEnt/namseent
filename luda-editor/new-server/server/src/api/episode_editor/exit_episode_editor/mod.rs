use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::exit_episode_editor::*;

pub async fn exit_episode_editor(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
