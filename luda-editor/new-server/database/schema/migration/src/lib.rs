mod latest_version;
mod migrate_0_1;

use anyhow::Result;
pub use latest_version::*;

pub async fn migrate(from_version: usize, db_conn: impl DatabaseConnection) -> Result<()> {
    assert!(from_version <= LATEST_VERSION);
    if from_version == LATEST_VERSION {
        return Ok(());
    }
    if LATEST_VERSION + 2 <= from_version {
        eprintln!("WARN: version gap is greater than 1. {from_version} -> {LATEST_VERSION}");
    }

    match from_version {
        0 => {
            migrate_0_1::migrate(db_conn)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

pub trait DatabaseConnection {
    fn map<From: document::Document, To: document::Document>(
        &self,
        f: impl FnMut(From) -> To,
    ) -> Result<()>;
}
