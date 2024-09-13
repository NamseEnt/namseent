use anyhow::Result;
#[allow(unused_imports)]
use migration::schema::*;
use rusqlite::Connection;
use std::{collections::HashMap, time::SystemTime};

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
    //     sk BLOB NOT NULL,
    //     value BLOB,
    //     version INTEGER,
    //     expired_at INTEGER,
    //     PRIMARY KEY (pk, sk)
    // )

    let debug_log_value_map = document::inventory::iter::<document::DocumentLogPlugin>
        .into_iter()
        .map(|plugin| (plugin.name, plugin.debug_log_value))
        .collect::<HashMap<_, _>>();

    let mut stmt = conn.prepare(
        "
    SELECT 
        name,
        value,
        expired_at,
        (expired_at = 0 OR expired_at >= unixepoch()),
        pk,
        sk
    FROM
        documents
        ",
    )?;

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let value: Vec<u8> = row.get(1)?;
        let expired_at: i64 = row.get(2)?;
        let expired: bool = !row.get(3)?;
        let pk: Vec<u8> = row.get(4)?;
        let sk: Option<Vec<u8>> = row.get(5)?;

        println!("--------");

        if let Some(debug_log_value) = debug_log_value_map.get(&name.as_ref()) {
            debug_log_value(&value)
        } else {
            println!("Unknown doc: {}: {:?}", name, value);
        }

        println!(
            "Expired({expired}): {}",
            if expired_at == 0 {
                "never".to_string()
            } else {
                format!(
                    "{}s",
                    expired_at
                        - SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)?
                            .as_secs() as i64
                )
            }
        );
        
        println!("PK: {:?}", pk);
        println!("SK: {:?}", sk);
    }

    Ok(())
}
