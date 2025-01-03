//! # Naming
//! - `Doc` suffix: A struct that represents a document in the database.
//! - `{A}To{B}Doc`: A struct that represents, A has many B.
//!
//! # Warning
//! - Don't change PKs's order.
//!     - PKs are serialized in the order of fields.
//!
//! # Auto Public Visibility
//! - All [document] structs and their fields are public by macro.
//!

mod asset;
mod auth;
mod episode;
mod insert_ordered_map;
mod scene;
mod team;
mod team_invite;
mod translation;

pub use asset::*;
pub use auth::*;
pub use episode::*;
pub use insert_ordered_map::*;
pub use namui_type::*;
pub use scene::*;
pub use team::*;
pub use team_invite::*;
pub use translation::*;

use document::*;
