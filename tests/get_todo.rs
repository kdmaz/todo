mod utils;
use serde_json::json;
use utils::spawn_app;

use crate::utils::insert_todos;

#[actix_rt::test]
async fn get_todo_404() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .get(&format!(
                "{}/api/todos/44444444-4444-4444-4444-444444444444",
                &test_app.address
            ))
            .send()
            .await
            .expect("Failed to execute request to get todo");

        assert!(response.status().is_client_error());
        assert_eq!(response.status().as_u16(), 404);
    })
    .await;
}

#[actix_rt::test]
async fn get_todo() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .get(&format!(
                "{}/api/todos/11111111-1111-1111-1111-111111111111",
                &test_app.address
            ))
            .send()
            .await
            .expect("Failed to execute request to get todo");

        assert!(response.status().is_success());
        let body = response.text().await.expect("Failed to get body");
        assert_eq!(
            body,
            json!(
                {
                    "id": "11111111-1111-1111-1111-111111111111",
                    "task": "task 2",
                    "complete": false
                }
            )
            .to_string()
        );
    })
    .await;
}
