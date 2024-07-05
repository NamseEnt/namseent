//! # Naming
//! - `Doc` suffix: A struct that represents a document in the database.
//! - `{A}To{B}Doc`: A struct that represents, A has many B.
//!
//! # Warning
//! - Don't change PKs's order.
//!     - PKs are serialized in the order of fields.
//!
//! # Auto Public Visibility
//! - All [schema] structs and their fields are public by macro.
//!

mod auth;
mod episode;
mod project;
mod team;
mod team_invite;

pub use auth::*;
pub use episode::*;
pub use project::*;
pub use team::*;
pub use team_invite::*;

use document::*;
use std::time::SystemTime;
