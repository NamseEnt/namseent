use super::{HeapArchived, KsStore};
use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::{rusqlite::OptionalExtension, SqliteConnectionManager};
use rkyv::ser::serializers::AllocSerializer;
use std::future::Future;

pub struct SqliteKsStore {
    pool: Pool<SqliteConnectionManager>,
}
impl SqliteKsStore {
    pub fn new() -> Self {
        const DB_PATH: &str = "db.sqlite";
        let manager = SqliteConnectionManager::file(DB_PATH).with_init(|c| {
            c.execute(
                "CREATE TABLE IF NOT EXISTS kv_store (key TEXT PRIMARY KEY, value BLOB, version INTEGER)",
                [],
            )
            .map(|_| ())
        });

        Self {
            pool: Pool::new(manager).unwrap(),
        }
    }
}

fn test() {
    let store = SqliteKsStore::new();
    #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
    struct MyType {
        a: u32,
        b: String,
    }
    let my_type = store.get::<MyType>("Hello").unwrap().unwrap();

    let my_type = my_type.deserialize();
}
impl KsStore for SqliteKsStore {
    fn get<T: rkyv::Archive>(&self, key: impl AsRef<str>) -> Result<Option<HeapArchived<T>>> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM kv_store WHERE key = ?")?;
        let vec: Option<Vec<_>> = stmt
            .query_row([key.as_ref()], |row| row.get(0))
            .optional()?;

        Ok(vec.map(|vec| HeapArchived::new(vec)))
    }

    fn put<T: rkyv::Serialize<AllocSerializer<0>>>(
        &self,
        key: impl AsRef<str>,
        value: &T,
    ) -> Result<()> {
        let conn = self.pool.get().unwrap();
        let mut stmt =
            conn.prepare("INSERT OR REPLACE INTO kv_store (key, value, version) VALUES (?, ?, 0)")?;
        let buffer = rkyv::to_bytes(value)?;
        assert_eq!(stmt.execute((key.as_ref(), buffer.as_slice()))?, 1);

        Ok(())
    }

    fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("DELETE FROM kv_store WHERE key = ?")?;
        stmt.execute([key.as_ref()])?;

        Ok(())
    }

    async fn update<T, Fut>(
        &self,
        key: impl AsRef<str>,
        update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    ) -> Result<bool>
    where
        T: rkyv::Archive + rkyv::Serialize<AllocSerializer<0>>,
        Fut: Future<Output = Option<Option<T>>>,
    {
        let conn = self.pool.get().unwrap();

        let output: Option<(Vec<_>, usize)> = {
            let mut stmt = conn.prepare("SELECT (value, version) FROM kv_store WHERE key = ?")?;
            stmt.query_row([key.as_ref()], |row| Ok((row.get(0)?, row.get(1)?)))
                .optional()?
        };
        let (old_value, version) = match output {
            Some((vec, version)) => (Some(HeapArchived::new(vec)), Some(version)),
            None => (None, None),
        };

        let Some(new_value) = update(old_value).await else {
            return Ok(true);
        };

        let Some(new_value) = new_value else {
            let mut stmt = conn.prepare("DELETE FROM kv_store WHERE key = ?")?;
            stmt.execute([key.as_ref()])?;
            return Ok(true);
        };

        let buffer = rkyv::to_bytes(&new_value)?;

        if let Some(version) = version {
            let mut stmt = conn.prepare(
                "UPDATE kv_store SET value = ?, version = version + 1 WHERE key = ? AND version = ?",
            )?;
            let changed = stmt.execute((buffer.as_slice(), key.as_ref(), version))?;
            return Ok(changed == 0);
        }

        let mut stmt =
            conn.prepare("INSERT INTO kv_store (key, value, version) VALUES (?, ?, 0)")?;

        match stmt.execute((key.as_ref(), buffer.as_slice())) {
            Ok(count) => {
                assert_eq!(count, 1);
                Ok(true)
            }
            Err(err) => {
                if let r2d2_sqlite::rusqlite::Error::SqliteFailure(err, _) = err {
                    if let r2d2_sqlite::rusqlite::ErrorCode::ConstraintViolation = err.code {
                        return Ok(false);
                    }
                }
                Err(err.into())
            }
        }
    }
}
