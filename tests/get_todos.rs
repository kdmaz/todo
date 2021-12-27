mod utils;
use crate::utils::insert_todos;
use serde_json::json;
use utils::spawn_app;

#[actix_rt::test]
async fn get_empty_todos() {
    spawn_app(|test_app| async move {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/todos", &test_app.address))
            .send()
            .await
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_success());
        let body = response.text().await.expect("Failed to get body");
        assert_eq!(body, "[]")
    })
    .await;
}

#[actix_rt::test]
async fn get_todos() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/todos", &test_app.address))
            .send()
            .await
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_success());
        let body = response.text().await.expect("Failed to get body");
        assert_eq!(
            body,
            json!([
                {
                    "id": "00000000-0000-0000-0000-000000000000",
                    "task": "task 1",
                    "complete": true
                },
                {
                    "id": "11111111-1111-1111-1111-111111111111",
                    "task": "task 2",
                    "complete": false
                },
                {
                    "id": "22222222-2222-2222-2222-222222222222",
                    "task": "task 3",
                    "complete":true
                }
            ])
            .to_string()
        )
    })
    .await;
}
