use tide;

use crate::db;
use crate::server::{bad_request, created, not_found, ok_json, Request};
use crate::server::{Group, Participant};

pub(crate) async fn ping(_req: Request) -> tide::Result {
    Ok("pong".into())
}

pub(crate) async fn fortune(req: Request) -> tide::Result {
    let db_pool = &req.state().db_pool;
    let fortune_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };

    let fortune = db::get_fortune_by_id(db_pool, fortune_id).await?;
    match fortune {
        Some(f) => Ok(f.into()),
        None => not_found(format!("No fortune found for id {fortune_id}")),
    }
}

pub(crate) async fn new_participant(mut req: Request) -> tide::Result {
    let participant: Participant = req.body_json().await?;
    let db_pool = &req.state().db_pool;
    db::insert_participant(db_pool, &participant).await?;
    created()
}

pub(crate) async fn get_participant(req: Request) -> tide::Result {
    let db_pool = &req.state().db_pool;
    let participant_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };
    let participant = db::get_participant(db_pool, participant_id).await?;
    match participant {
        Some(p) => ok_json(&p),
        None => not_found(format!("No participant found for id {participant_id}")),
    }
}

pub(crate) async fn new_group(mut req: Request) -> tide::Result {
    let group: Group = req.body_json().await?;
    let db_pool = &req.state().db_pool;
    db::insert_group(db_pool, &group).await?;
    created()
}
