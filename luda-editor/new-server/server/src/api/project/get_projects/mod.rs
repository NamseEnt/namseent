use crate::*;
use database::schema::*;
use luda_rpc::project::get_projects::*;

pub async fn get_projects(
    ArchivedRequest { team_id }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
