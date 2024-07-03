//! `rpc_macro::define_rpc!` will automatically put `InternalServerError` in the `Error` enum.

mod types;

pub use rkyv;
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
        }
    },
    Project: {
        get_projects: {
            struct Request {
                team_id: String,
            }
            struct Response {
                projects: Vec<Project>,
            }
            enum Error {
            }
        }
    },
    Episode: {
        get_episodes: {
            struct Request {
                project_id: String,
            }
            struct Response {
                episodes: Vec<Episode>,
            }
            enum Error {
            }
        }
    },
}
