use crate::*;
use database::Database;
pub enum HandleResult {
    Response(Vec<u8>),
    Error(Vec<u8>),
}
pub async fn handle(
    api_index: u16,
    in_payload: &[u8],
    db: Database,
    session: Session,
) -> Result<HandleResult> {
    match api_index {
        0u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::auth::google_auth::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::auth::google_auth::google_auth(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        1u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::auth::session_token_auth::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::auth::session_token_auth::session_token_auth(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        2u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::auth::revoke_session_token::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::auth::revoke_session_token::revoke_session_token(request, db, session).await
            {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        3u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team::get_my_teams::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team::get_my_teams::get_my_teams(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        4u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::project::get_projects::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::project::get_projects::get_projects(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        5u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::episode::get_episodes::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::episode::get_episodes::get_episodes(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(
                    rkyv::to_bytes::<_, 64>(&response)?.to_vec(),
                )),
                Err(error) => Ok(HandleResult::Error(
                    rkyv::to_bytes::<_, 64>(&error)?.to_vec(),
                )),
            }
        }
        _ => Err(anyhow::anyhow!("Unknown packet type: {}", api_index)),
    }
}
