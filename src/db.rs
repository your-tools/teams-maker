use sqlx::{migrate, query, query_as, Error, SqlitePool};

use crate::server::{Group, Participant};

pub(crate) async fn migrate_db(db: &SqlitePool) -> Result<(), Error> {
    migrate!("src/migrations").run(db).await?;
    Ok(())
}

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
    match participant.group_id {
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
    };
    Ok(())
}

pub(crate) async fn insert_group(db: &SqlitePool, group: &Group) -> Result<(), Error> {
    query!("INSERT INTO groups(name) VALUES(?)", group.name)
        .execute(db)
        .await?;
    Ok(())
}

pub(crate) async fn get_participants(
    db: &SqlitePool,
    group_id: i64,
) -> Result<Vec<Participant>, Error> {
    let query = query_as!(
        Participant,
        "SELECT name, group_id FROM participants WHERE group_id=?",
        group_id
    );
    let res = query.fetch_all(db).await?;
    Ok(res)
}
