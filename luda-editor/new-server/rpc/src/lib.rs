#![allow(clippy::enum_variant_names)]
//! `rpc_macro::define_rpc!` will automatically put `InternalServerError` in the `Error` enum.

mod types;

use macro_common_lib::*;
pub use rkyv;
use std::collections::HashMap;
pub use types::*;

rpc_macro::define_rpc! {
    Auth: {
        google_auth: {
            struct Request {
                jwt: String,
            }
            struct Response {
                session_token: String,
            }
            enum Error {
                AlreadyLoggedIn,
            }
        },
        session_token_auth: {
            struct Request {
                session_token: String,
            }
            struct Response {
            }
            enum Error {
                AlreadyLoggedIn,
                SessionTokenNotExist,
            }
        },
        revoke_session_token: {
            struct Request {
                session_token: String,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
            }
        },
    },
    Team: {
        get_my_teams: {
            struct Request {
            }
            struct Response {
                teams: Vec<Team>,
            }
            enum Error {
                NeedLogin,
            }
        },
        create_new_team: {
            struct Request {
                name: String,
            }
            struct Response {
                team_id: String,
            }
            enum Error {
                NeedLogin,
                TooManyTeams,
                DuplicatedName,
            }
        },
    },
    TeamInvite: {
        join_team: {
            struct Request {
                code: String,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                InvalidCode,
            }
        },
        create_team_invite_code: {
            struct Request {
                team_id: String,
            }
            struct Response {
                code: TeamInviteCode,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                TooManyCodes,
            }
        },
        list_team_invite_codes: {
            struct Request {
                team_id: String,
            }
            struct Response {
                codes: Vec<TeamInviteCode>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
        invalidate_team_invite_code: {
            struct Request {
                team_id: String,
                code: String,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
    },
    Project: {
        create_new_project: {
            struct Request {
                team_id: String,
                name: String,
            }
            struct Response {
                project_id: String,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                TooManyProjects,
                DuplicatedName,
            }
        },
        get_projects: {
            struct Request {
                team_id: String,
            }
            struct Response {
                projects: Vec<Project>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
    },
    Episode: {
        create_new_episode: {
            struct Request {
                project_id: String,
                name: String,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
            }
        },
        get_episodes: {
            struct Request {
                project_id: String,
            }
            struct Response {
                episodes: Vec<Episode>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
            }
        },
    },
    EpisodeEditor: {
        join_episode_editor: {
            struct Request {
                episode_id: String,
            }
            struct Response {
                scenes: Vec<Scene>,
                /// key: (scene_id, language_code)
                texts: HashMap<String, HashMap<String, String>>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
                OtherUserEditing,
            }
        },
        exit_episode_editor: {
            struct Request {
                episode_id: String,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
            }
        },
        try_edit_episode: {
            struct Request {
                episode_id: String,
                action: EpisodeEditAction,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
                ImpossibleAction,
                YouDoNotHaveEditorLock,
                InvalidSceneIndex,
            }
        },
        load_speaker_slots: {
            struct Request {
                episode_id: String,
            }
            struct Response {
                speaker_ids: Vec<String>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
            }
        },
        save_speaker_slots: {
            struct Request {
                episode_id: String,
                speaker_ids: Vec<String>,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
        get_speaker_names: {
            struct Request {
                project_id: String,
                speaker_ids: Vec<String>,
                language_code: String,
            }
            struct Response {
                speaker_names: Vec<Option<String>>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
    },
    Asset: {
        reserve_team_asset_upload: {
            struct Request {
                team_id: String,
                asset_name: String,
                byte_size: u64,
                asset_kind: migration::schema::AssetKind,
            }
            struct Response {
                asset_id: String,
                presigned_put_uri: String,
                headers: Vec<(String, String)>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                NotEnoughSpace,
            }
        },
    },
}
