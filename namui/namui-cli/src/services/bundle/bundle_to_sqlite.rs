use super::*;
use anyhow::Result;
use rayon::prelude::*;
use rusqlite::{Connection, DatabaseName, OptionalExtension};
use std::{
    collections::HashSet,
    fs::create_dir_all,
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::UNIX_EPOCH,
};

pub fn bundle_to_sqlite(
    sqlite_path: impl AsRef<std::path::Path>,
    collect_operations: Vec<CollectOperation>,
) -> Result<()> {
    let sqlite_path = sqlite_path.as_ref().to_path_buf();
    create_dir_all(sqlite_path.parent().unwrap())?;
    let create_conn = || Connection::open(&sqlite_path).unwrap();
    let conn = create_conn();
    let changed = Arc::new(AtomicBool::new(false));

    conn.execute(
        "CREATE TABLE IF NOT EXISTS bundle (
                path TEXT PRIMARY KEY NOT NULL,
                data BLOB,
                modified INTEGER
            )",
        (),
    )?;

    let bundle_dest_list = collect_operations
        .iter()
        .map(|operation| operation.dest_path().to_str().unwrap().to_string())
        .collect::<HashSet<_>>();

    {
        let mut stmt = conn.prepare("SELECT path FROM bundle")?;
        for path in stmt.query_map([], |row| row.get::<usize, String>(0))? {
            let path = path?;
            if !bundle_dest_list.contains(&path) {
                conn.execute("DELETE FROM bundle WHERE path = ?", [&path])?;
                changed.store(true, Ordering::Relaxed);
            }
        }
    };

    collect_operations.into_par_iter().try_for_each_init(
        create_conn,
        |conn, operation| -> Result<()> {
            let metadata = operation.src_path.metadata()?;
            assert!(metadata.is_file());

            let modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();

            let stored_modified = conn
                .query_row(
                    "SELECT modified FROM bundle WHERE path = ?",
                    [&operation.dest_path().to_str().unwrap()],
                    |row| row.get::<usize, u64>(0),
                )
                .optional()?;

            if stored_modified == Some(modified) {
                return Ok(());
            }

            changed.store(true, Ordering::Relaxed);

            conn.execute(
                "INSERT OR REPLACE INTO bundle (path, data, modified) VALUES (?, ZEROBLOB(?), ?)",
                (
                    operation.dest_path().to_str().unwrap(),
                    metadata.len(),
                    modified,
                ),
            )?;

            let rowid = conn.last_insert_rowid();

            let mut blob = conn.blob_open(DatabaseName::Main, "bundle", "data", rowid, false)?;

            let mut file = std::fs::File::open(&operation.src_path)?;

            io::copy(&mut file, &mut blob)?;

            blob.close()?;

            Ok(())
        },
    )?;

    if changed.load(Ordering::Relaxed) {
        conn.execute("VACUUM", ())?;
    }

    Ok(())
}
