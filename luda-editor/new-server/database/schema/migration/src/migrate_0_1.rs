use crate::*;
use anyhow::Result;

///
/// - [Edit] MyDocument
///   - new field: tags: Vec<String>
///
pub fn migrate(db_conn: impl DatabaseConnection) -> Result<()> {
    db_conn.map(|from: schema_0::MyDocument| schema_1::MyDocument {
        content: from.content,
        name: from.name,
        tags: vec![],
    })?;
    Ok(())
}
