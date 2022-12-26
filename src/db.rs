use sqlx::Error;
use sqlx::SqlitePool;

use crate::server::ParticipantResponse;

pub(crate) async fn get_fortune_by_id(db: &SqlitePool, id: i64) -> Result<Option<String>, Error> {
    let query = sqlx::query!("SELECT text FROM fortunes WHERE id = ?", id);
    let res = query.fetch_optional(db).await?;
    return Ok(match res {
        Some(record) => Some(record.text),
        None => None,
    });
}

pub(crate) async fn get_participant(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<ParticipantResponse>, Error> {
    let query = sqlx::query!("SELECT name, group_id FROM participants WHERE id = ?", id);
    let res = query.fetch_optional(db).await?;
    return Ok(match res {
        Some(record) => Some(ParticipantResponse {
            group_id: record.group_id,
            name: record.name,
        }),
        None => None,
    });
}
