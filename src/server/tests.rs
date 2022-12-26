use serde::Serialize;
use tide::http::{Method, Request, Response, Url};
use tide::{Body, StatusCode};

use crate::server::{Group, Participant};

use super::{get_app, Server};

async fn get_test_app() -> Server {
    let app = get_app("sqlite://:memory:").await.unwrap();
    app
}

async fn get(app: &Server, path: &str) -> String {
    let url = Url::parse(&format!("http://example.com{path}")).unwrap();
    let request = Request::new(Method::Get, url);
    let mut response: Response = app.respond(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    response.body_string().await.unwrap()
}

async fn post_and_create<T>(app: &Server, path: &str, payload: T) -> String
where
    T: Serialize,
{
    let url = Url::parse(&format!("http://example.com{path}")).unwrap();
    let mut request = Request::new(Method::Post, url);
    request.set_body(Body::from_json(&payload).unwrap());
    request.insert_header("Content-Type", "application/json");
    let mut response: Response = app.respond(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);
    response.body_string().await.unwrap()
}

#[async_std::test]
async fn test_ping() {
    let app = get_test_app().await;

    let body = get(&app, "/ping").await;

    assert_eq!(body, "pong");
}

#[async_std::test]
async fn test_fortune() {
    let app = get_test_app().await;

    let body = get(&app, "/fortune/1").await;

    assert_eq!(body, "hello there");
}

#[async_std::test]
async fn test_create_participant() {
    let app = get_test_app().await;

    let bob = Participant {
        name: "Bob".to_string(),
        group_id: None,
    };
    post_and_create(&app, "/participant", bob).await;
}

#[async_std::test]
async fn test_create_group() {
    let app = get_test_app().await;

    let group = Group {
        name: "Group 1".to_string(),
    };
    post_and_create(&app, "/group", group).await;
}

#[async_std::test]
async fn test_create_participant_in_group() {
    let app = get_test_app().await;

    let group = Group {
        name: "Group 1".to_string(),
    };
    post_and_create(&app, "/group", group).await;

    let bob = Participant {
        name: "Bob".to_string(),
        group_id: Some(1),
    };
    post_and_create(&app, "/participant", bob).await;

    let body = get(&app, "/participant/1").await;
    let desc: Participant = serde_json::from_str(&body).unwrap();
    assert_eq!(desc.group_id, Some(1));
}
