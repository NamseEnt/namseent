use crate::*;
use database::schema::*;
use luda_rpc::episode::get_episodes::*;

pub async fn get_episodes(
    ArchivedRequest { project_id }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
