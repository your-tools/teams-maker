use sqlx::Error;
use sqlx::SqlitePool;
use sqlx::{query, query_as};

use crate::server::{Group, Participant};

pub(crate) async fn get_fortune_by_id(db: &SqlitePool, id: i64) -> Result<Option<String>, Error> {
    let query = query!("SELECT text FROM fortunes WHERE id = ?", id);
    let res = query.fetch_optional(db).await?;
    Ok(match res {
        Some(record) => Some(record.text),
        None => None,
    })
}

pub(crate) async fn get_participant(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<Participant>, Error> {
    let query = query_as!(
        Participant,
        "SELECT name, group_id FROM participants WHERE id = ?",
        id
    );
    let res = query.fetch_optional(db).await?;
    Ok(res)
}

pub(crate) async fn insert_participant(
    db: &SqlitePool,
    participant: &Participant,
) -> Result<(), Error> {
    Ok(match participant.group_id {
        None => {
            query!("INSERT INTO participants(name) VALUES(?)", participant.name)
                .execute(db)
                .await?;
        }
        Some(group_id) => {
            query!(
                "INSERT INTO participants(name, group_id) VALUES (?, ?)",
                participant.name,
                group_id,
            )
            .execute(db)
            .await?;
        }
    })
}

pub(crate) async fn insert_group(db: &SqlitePool, group: &Group) -> Result<(), Error> {
    query!("INSERT INTO groups(name) VALUES(?)", group.name)
        .execute(db)
        .await?;
    Ok(())
}
