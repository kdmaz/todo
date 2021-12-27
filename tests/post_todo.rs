mod utils;
use crate::utils::insert_todos;
use serde_json::json;
use todo::Todo;
use utils::spawn_app;

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

        let todo_response = serde_json::from_str(body.as_str())
            .expect("Failed to get Todo from JSON response body");

        let saved_todo = sqlx::query_as!(
            Todo,
            "SELECT id, task, complete FROM todo WHERE task = 'new task'"
        )
        .fetch_one(&test_app.pool)
        .await
        .expect("Failed to fetch saved todo");

        assert_eq!(saved_todo, todo_response);
    })
    .await;
}
