mod utils;
use crate::utils::insert_todos;
use serde_json::json;
use utils::spawn_app;
use uuid::Uuid;

#[actix_rt::test]
async fn post_todo() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/api/todos", &test_app.address))
            .json(&json!({
                "task": "new task",
                "complete": true
            }))
            .send()
            .await
            .expect("Failed to execute request to post todo");

        assert!(response.status().is_success());
        let body = response.text().await.expect("Failed to get body");

        let mut split_body = body.split(",");
        let id_prop = split_body.next().expect("Failed to get id key/value pair");
        let task_prop = split_body
            .next()
            .expect("Failed to get task key/value pair");
        let complete_prop = split_body
            .next()
            .expect("Failed to get complete key/value pair");

        let mut id_prop = id_prop.split(":");
        let id_key = id_prop.next().expect("Failed to get id key");
        let id_value = id_prop
            .next()
            .expect("Failed to get id value")
            .replace("\"", "");
        let id_value = id_value.as_str();

        assert_eq!(id_key, r#"{"id""#);
        Uuid::parse_str(id_value).expect("Failed to parse str to uuid");
        assert_eq!(task_prop, r#""task":"new task""#);
        assert_eq!(complete_prop, r#""complete":true}"#);
    })
    .await;
}
