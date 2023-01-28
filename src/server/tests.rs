use serde::Serialize;
use tide::http::{Method, Request, Response, Url};
use tide::{Body, StatusCode};

use crate::db::{insert_group, insert_participant};
use crate::server::{Group, Participant};
use crate::Team;

use super::{get_app, Server};

struct TestApp {
    app: Server,
}

impl TestApp {
    async fn new() -> Self {
        let app = get_app("sqlite://:memory:").await.unwrap();
        Self { app }
    }

    async fn get(&self, path: &str) -> String {
        let url = Url::parse(&format!("http://example.com{path}")).unwrap();
        let request = Request::new(Method::Get, url);
        let mut response: Response = (&self.app).respond(request).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::Ok,
            "{:?}",
            response.body_string().await
        );
        response.body_string().await.unwrap()
    }

    async fn post_and_create<T>(&self, path: &str, payload: T) -> String
    where
        T: Serialize,
    {
        let url = Url::parse(&format!("http://example.com{path}")).unwrap();
        let mut request = Request::new(Method::Post, url);
        request.set_body(Body::from_json(&payload).unwrap());
        request.insert_header("Content-Type", "application/json");
        let mut response: Response = (&self.app).respond(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::Created);
        response.body_string().await.unwrap()
    }

    async fn with_group(self, name: &str) -> Self {
        let db_pool = &self.app.state().db_pool;
        insert_group(
            &db_pool,
            &Group {
                name: name.to_string(),
            },
        )
        .await
        .unwrap();
        self
    }

    async fn with_participants_in_group(self, group_id: i64, names: &[&str]) -> Self {
        let db_pool = &self.app.state().db_pool;
        for name in names {
            let participant = Participant {
                name: name.to_string(),
                group_id: Some(group_id),
            };
            insert_participant(&db_pool, &participant).await.unwrap();
        }
        self
    }
}

#[async_std::test]
async fn test_ping() {
    let test_app = TestApp::new().await;

    let body = test_app.get("/ping").await;

    assert_eq!(body, "pong");
}

#[async_std::test]
async fn test_fortune() {
    let test_app = TestApp::new().await;

    let body = test_app.get("/fortune/1").await;

    assert_eq!(body, "hello there");
}

#[async_std::test]
async fn test_create_participant() {
    let test_app = TestApp::new().await;
    let bob = Participant {
        name: "Bob".to_string(),
        group_id: None,
    };
    test_app.post_and_create("/participant", bob).await;
}

#[async_std::test]
async fn test_create_group() {
    let test_app = TestApp::new().await;

    let group = Group {
        name: "Group 1".to_string(),
    };
    test_app.post_and_create("/group", group).await;
}

#[async_std::test]
async fn test_create_participant_in_group() {
    let test_app = TestApp::new().await.with_group("students").await;

    let bob = Participant {
        name: "Bob".to_string(),
        group_id: Some(1),
    };
    test_app.post_and_create("/participant", bob).await;

    let body = test_app.get("/participant/1").await;
    let desc: Participant = serde_json::from_str(&body).unwrap();
    assert_eq!(desc.group_id, Some(1));
}

#[async_std::test]
async fn test_create_teams() {
    let test_app = TestApp::new()
        .await
        .with_group("group 1")
        .await
        .with_participants_in_group(1, &["Alice", "Bob", "Charlie", "Dave"])
        .await;

    let body = test_app.get("/teams?group=1&team_size=2").await;
    let teams: Vec<Team> = serde_json::from_str(&body).unwrap();
    assert_eq!(teams.len(), 2, "teams: {:?}", teams);
}
