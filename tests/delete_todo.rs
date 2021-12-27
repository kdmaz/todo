mod utils;
use crate::utils::insert_todos;
use todo::Todo;
use utils::spawn_app;

#[actix_rt::test]
async fn delete_todo_404() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .delete(&format!(
                "{}/api/todos/44444444-4444-4444-4444-444444444444",
                &test_app.address
            ))
            .send()
            .await
            .expect("Failed to execute request to delete todo");

        assert!(response.status().is_client_error());
        assert_eq!(response.status().as_u16(), 404);
    })
    .await;
}

#[actix_rt::test]
async fn delete_todo() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .delete(&format!(
                "{}/api/todos/11111111-1111-1111-1111-111111111111",
                &test_app.address
            ))
            .send()
            .await
            .expect("Failed to execute request to delete todo");

        assert!(response.status().is_success());

        let saved_todo = sqlx::query_as!(
            Todo,
            "SELECT id, task, complete FROM todo WHERE id = '11111111-1111-1111-1111-111111111111'"
        )
        .fetch_one(&test_app.pool)
        .await;

        assert!(saved_todo.is_err());
    })
    .await;
}
