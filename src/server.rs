use dotenv;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tide::{log::*, Body, Response, StatusCode};

use crate::db;

pub(crate) mod routes;

type Request = tide::Request<State>;
pub type Server = tide::Server<State>;

#[derive(Clone)]
pub struct State {
    db_pool: SqlitePool,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Participant {
    pub(crate) name: String,
    pub(crate) group_id: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Group {
    pub name: String,
}

fn created() -> tide::Result {
    Ok(Response::builder(StatusCode::Created).build())
}

fn bad_request(mesage: String) -> tide::Result {
    Ok(Response::builder(StatusCode::BadRequest)
        .body(Body::from_string(mesage))
        .build())
}

fn not_found(mesage: String) -> tide::Result {
    Ok(Response::builder(StatusCode::NotFound)
        .body(Body::from_string(mesage))
        .build())
}

fn ok_json(payload: &impl Serialize) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(payload)?)
        .build())
}

async fn get_app(db_url: &str) -> tide::Result<Server> {
    info!("Using database at {db_url}");
    let pool = SqlitePool::connect(db_url).await?;
    db::migrate_db(&pool).await?;

    let mut app = tide::with_state(State { db_pool: pool });

    app.at("/ping").get(routes::ping);
    app.at("/fortune/:id").get(routes::fortune);
    app.at("/participant").post(routes::new_participant);
    app.at("/participant/:id").get(routes::get_participant);
    app.at("/group").post(routes::new_group);
    app.at("/teams").get(routes::get_teams);
    Ok(app)
}

pub async fn run() -> tide::Result<()> {
    let _ = dotenv::dotenv();
    tide::log::start();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL not set");
        std::process::exit(1)
    });
    let app = get_app(&db_url).await?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[cfg(test)]
mod tests;
