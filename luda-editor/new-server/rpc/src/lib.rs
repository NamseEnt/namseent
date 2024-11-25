#![allow(clippy::enum_variant_names)]
//! `rpc_macro::define_rpc!` will automatically put `InternalServerError` in the `Error` enum.

mod types;

use macro_common_lib::*;
pub use rkyv;
pub use types::*;

rpc_macro::define_rpc! {
    Auth: {
        google_auth: {
            struct Request {
                jwt: String,
            }
            struct Response {
                session_token: u128,
            }
            enum Error {
                AlreadyLoggedIn,
            }
        },
        session_token_auth: {
            struct Request {
                session_token: u128,
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
                session_token: u128,
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
                UserNotExist,
            }
        },
        create_new_team: {
            struct Request {
                name: String,
            }
            struct Response {
                team_id: u128,
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
                code: u128,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                InvalidCode,
                AlreadyJoined,
            }
        },
        create_team_invite_code: {
            struct Request {
                team_id: u128,
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
                team_id: u128,
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
                team_id: u128,
                code: u128,
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
                team_id: u128,
                name: String,
            }
            struct Response {
                project_id: u128,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                TooManyProjects,
            }
        },
        get_projects: {
            struct Request {
                team_id: u128,
            }
            struct Response {
                projects: Vec<Project>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
        put_speaker: {
            struct Request {
                project_id: u128,
                speaker: Speaker,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
                TeamNotExist,
            }
        },
        list_speakers: {
            struct Request {
                project_id: u128,
            }
            struct Response {
                speakers: Vec<Speaker>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
                TeamNotExist,
            }
        },
        delete_speaker: {
            struct Request {
                project_id: u128,
                speaker_id: u128,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
                TeamNotExist,
            }
        },
    },
    Episode: {
        create_new_episode: {
            struct Request {
                project_id: u128,
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
                project_id: u128,
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
                episode_id: u128,
            }
            struct Response {
                scenes: Vec<Scene>,
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
                episode_id: u128,
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
                episode_id: u128,
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
                SceneNotExist,
            }
        },
        load_speaker_slots: {
            struct Request {
                episode_id: u128,
            }
            struct Response {
                speaker_ids: Vec<u128>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
            }
        },
        save_speaker_slots: {
            struct Request {
                episode_id: u128,
                speaker_ids: Vec<u128>,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                EpisodeNotExist,
            }
        },
        get_speaker_names: {
            struct Request {
                project_id: u128,
                speaker_ids: Vec<u128>,
                language_code: String,
            }
            struct Response {
                speaker_names: Vec<Option<String>>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                ProjectNotExist,
            }
        },
    },
    Asset: {
        reserve_team_asset_upload: {
            struct Request {
                team_id: u128,
                asset_name: String,
                byte_size: u64,
                asset_kind: AssetKind,
                tags: Vec<AssetTag>,
            }
            struct Response {
                asset_id: u128,
                presigned_put_uri: String,
                headers: Vec<(String, String)>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
                NotEnoughSpace,
            }
        },
        get_team_asset_docs: {
            struct Request {
                team_id: u128,
            }
            struct Response {
                asset_docs: Vec<AssetDoc>,
            }
            enum Error {
                NeedLogin,
                PermissionDenied,
            }
        },
        update_asset_tags_for_asset: {
            struct Request {
                asset_id: u128,
                tags: Vec<AssetTag>,
            }
            struct Response {
            }
            enum Error {
                NeedLogin,
                AssetNotExist,
                PermissionDenied,
            }
        },
    },
}
