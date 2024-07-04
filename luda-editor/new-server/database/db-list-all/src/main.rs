use anyhow::Result;
#[allow(unused_imports)]
use migration::schema::*;
use rusqlite::Connection;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await?;
    Ok(())
}

async fn real_main() -> Result<()> {
    let db_path = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("missing db path"))?;
    let conn = Connection::open(db_path)?;

    // CREATE TABLE IF NOT EXISTS
    // documents (
    //     name TEXT NOT NULL,
    //     pk BLOB NOT NULL,
    //     sk BLOB,
    //     value BLOB,
    //     version INTEGER,
    //     expired_at INTEGER,
    //     PRIMARY KEY (pk, sk)
    // )

    let debug_log_value_map = document::inventory::iter::<document::DocumentLogPlugin>
        .into_iter()
        .map(|plugin| (plugin.name, plugin.debug_log_value))
        .collect::<HashMap<_, _>>();

    let mut stmt = conn.prepare("SELECT * from documents")?;

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let value: Vec<u8> = row.get(3)?;

        if let Some(debug_log_value) = debug_log_value_map.get(&name.as_ref()) {
            debug_log_value(&value)
        } else {
            println!("Unknown doc: {}: {:?}", name, value);
        }
    }

    Ok(())
}
