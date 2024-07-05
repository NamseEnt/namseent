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
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        1u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::auth::session_token_auth::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::auth::session_token_auth::session_token_auth(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
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
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        3u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team::get_my_teams::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team::get_my_teams::get_my_teams(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        4u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team::create_new_team::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team::create_new_team::create_new_team(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        5u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team_invite::join_team::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team_invite::join_team::join_team(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        6u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team_invite::create_team_invite_code::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team_invite::create_team_invite_code::create_team_invite_code(
                request, db, session,
            )
            .await
            {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        7u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team_invite::list_team_invite_codes::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team_invite::list_team_invite_codes::list_team_invite_codes(
                request, db, session,
            )
            .await
            {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        8u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::team_invite::invalidate_team_invite_code::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::team_invite::invalidate_team_invite_code::invalidate_team_invite_code(
                request, db, session,
            )
            .await
            {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        9u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::project::get_projects::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::project::get_projects::get_projects(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        10u16 => {
            let Ok(request) = rkyv::validation::validators::check_archived_root::<
                luda_rpc::episode::get_episodes::Request,
            >(in_payload) else {
                return Err(anyhow::anyhow!("Failed to validate packet"));
            };
            match api::episode::get_episodes::get_episodes(request, db, session).await {
                Ok(response) => Ok(HandleResult::Response(serializer::serialize(&response)?)),
                Err(error) => Ok(HandleResult::Error(serializer::serialize(&error)?)),
            }
        }
        _ => Err(anyhow::anyhow!("Unknown packet type: {}", api_index)),
    }
}
