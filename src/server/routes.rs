use tide;
use tide::{Response, StatusCode};

use crate::db;
use crate::server::{bad_request, not_found, ok_json, Request};
use crate::server::{Group, ParticipantRequest};

pub(crate) async fn ping(_req: Request) -> tide::Result {
    Ok("pong".into())
}

pub(crate) async fn fortune(req: Request) -> tide::Result {
    let connection = &req.state().db;
    let fortune_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };

    let fortune = db::get_fortune_by_id(connection, fortune_id).await?;
    match fortune {
        Some(f) => return Ok(f.into()),
        None => not_found(format!("Not fortune found for id {fortune_id}")),
    }
}

pub(crate) async fn new_participant(mut req: Request) -> tide::Result {
    let participant: ParticipantRequest = req.body_json().await?;
    let connection = &req.state().db;
    match participant.group_id {
        None => {
            sqlx::query!("INSERT INTO participants(name) VALUES(?)", participant.name)
                .execute(connection)
                .await?;
        }
        Some(group_id) => {
            sqlx::query!(
                "INSERT INTO participants(name, group_id) VALUES (?, ?)",
                participant.name,
                group_id,
            )
            .execute(connection)
            .await?;
        }
    };
    Ok(Response::builder(StatusCode::Created).build())
}

pub(crate) async fn get_participant(req: Request) -> tide::Result {
    let db = &req.state().db;
    let participant_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };
    let description = db::get_participant(db, participant_id).await?;
    ok_json(&description)
}

pub(crate) async fn new_group(mut req: Request) -> tide::Result {
    let group: Group = req.body_json().await?;
    let query = sqlx::query!("INSERT INTO groups(name) VALUES(?)", group.name);
    let db = &req.state().db;
    let _ = query.execute(db).await?;
    Ok(Response::builder(StatusCode::Created).build())
}
