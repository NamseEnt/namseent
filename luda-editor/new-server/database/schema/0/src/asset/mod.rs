//! Sharing is only through the store
//! What is uploaded to the store cannot be deleted by the seller arbitrarily (exception: what has not been sold at all)
//! We can delete it due to store rules or legal issues. However, users will be notified that it has been deleted
//!
//! Assets = shared assets (=purchased from the store) and assets uploaded directly by our team
//! Any member of the team can delete the assets uploaded by the team.
//! Trash can function. What the team deleted is temporarily stored for 1 week by default, and can be forcibly deleted

mod tag;

use crate::*;
use std::collections::HashSet;
pub use tag::*;

#[document]
#[belongs_to(Team)]
struct AssetDoc {
    name: String,
    shared: bool,
    asset_kind: AssetKind,
    byte_size: u64,
    tags: HashSet<AssetTag>,
}

#[doc_part]
#[derive(Copy)]
enum AssetKind {
    Sprite,
    Audio,
}

#[document]
struct TeamAssetTotalBytesDoc {
    #[id]
    team_id: u128,
    limit_bytes: u64,
    used_bytes: u64,
}
