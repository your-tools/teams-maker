use dotenv;
use sqlx::SqlitePool;
use tide::{log::*, Body, Response, StatusCode};
type Request = tide::Request<State>;
pub type Server = tide::Server<State>;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct State {
    db: SqlitePool,
}

async fn ping(_req: Request) -> tide::Result {
    Ok("pong".into())
}

fn extract_number(req: &Request, param: &'static str) -> Result<i64, tide::Error> {
    let result = req.param(param)?.parse();
    match result {
        Ok(d) => Ok(d),
        Err(e) => Err(tide::Error::new(StatusCode::BadRequest, e)),
    }
}

async fn fortune(req: Request) -> tide::Result {
    let db = &req.state().db;
    let fortune_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };

    let query = sqlx::query!("SELECT text FROM fortunes WHERE id = ?", fortune_id);
    let res = query.fetch_optional(db).await?;
    if let Some(record) = res {
        return Ok(record.text.into());
    }

    not_found(format!("Not fortune found for id {fortune_id}"))
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Participant {
    pub name: String,
    pub group_id: Option<i64>,
}

async fn new_participant(mut req: Request) -> tide::Result {
    let participant: Participant = req.body_json().await?;
    let db = &req.state().db;
    match participant.group_id {
        None => {
            sqlx::query!("INSERT INTO participants(name) VALUES(?)", participant.name)
                .execute(db)
                .await?;
        }
        Some(group_id) => {
            sqlx::query!(
                "INSERT INTO participants(name, group_id) VALUES (?, ?)",
                participant.name,
                group_id,
            )
            .execute(db)
            .await?;
        }
    };
    Ok(Response::builder(StatusCode::Created).build())
}

#[derive(Serialize, Deserialize, Debug)]
struct ParticipantDescription {
    name: String,
    group_id: Option<i64>,
}

async fn get_participant(req: Request) -> tide::Result {
    let participant_id: i64 = match req.param("id")?.parse() {
        Ok(d) => d,
        Err(e) => return bad_request(e.to_string()),
    };
    let db = &req.state().db;
    let query = sqlx::query!(
        "SELECT name, group_id FROM participants WHERE id = ?",
        participant_id
    );
    let res = match query.fetch_optional(db).await? {
        Some(s) => s,
        None => return not_found(format!("No participant found with id {participant_id}")),
    };
    let name = res.name;
    let group_id = res.group_id;
    ok_json(&(ParticipantDescription { name, group_id }))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    pub name: String,
}

async fn new_group(mut req: Request) -> tide::Result {
    let group: Group = req.body_json().await?;
    let query = sqlx::query!("INSERT INTO groups(name) VALUES(?)", group.name);
    let db = &req.state().db;
    let _ = query.execute(db).await?;
    Ok(Response::builder(StatusCode::Created).build())
}

async fn get_app(db_url: &str) -> tide::Result<Server> {
    info!("Using database at {db_url}");
    let pool = SqlitePool::connect(&db_url).await?;

    let mut app = tide::with_state(State { db: pool });

    app.at("/ping").get(ping);
    app.at("/fortune/:id").get(fortune);
    app.at("/participant").post(new_participant);
    app.at("/participant/:id").get(get_participant);
    app.at("/group").post(new_group);
    Ok(app)
}

#[async_std::main]
pub async fn main() -> tide::Result<()> {
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
