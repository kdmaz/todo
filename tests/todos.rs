mod utils;
use serde_json::json;
use sqlx::PgPool;
use utils::spawn_app;

async fn insert_todos(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO todo (id, task, complete)
        VALUES 
        (
            '00000000-0000-0000-0000-000000000000',
            'task 1',
            true
        ),
        (
            '11111111-1111-1111-1111-111111111111',
            'task 2',
            false
        ),
        (
            '22222222-2222-2222-2222-222222222222',
            'task 3',
            true
        )"
    )
    .execute(pool)
    .await
    .expect("Failed to insert todos");
}

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
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_client_error());
        assert_eq!(response.status().as_u16(), 404);
    })
    .await;
}

#[actix_rt::test]
async fn get_todo_bad_uuid() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/todos/bad-uuid", &test_app.address))
            .send()
            .await
            .expect("Failed to execute request to get todos");

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
            .expect("Failed to execute request to get todos");

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

#[actix_rt::test]
async fn put_todo_bad_uuid() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .put(&format!("{}/api/todos/bad-uuid", &test_app.address))
            .body(r#"{"task":"updated task","complete":true}"#)
            .send()
            .await
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_client_error());
        assert_eq!(response.status().as_u16(), 404);
    })
    .await;
}

#[actix_rt::test]
async fn put_todo_404() {
    spawn_app(|test_app| async move {
        insert_todos(&test_app.pool).await;
        let client = reqwest::Client::new();
        let response = client
            .put(&format!(
                "{}/api/todos/44444444-4444-4444-4444-444444444444",
                &test_app.address
            ))
            .body(r#"{"task":"updated task","complete":true}"#)
            .send()
            .await
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_client_error());
        assert_eq!(response.status().as_u16(), 404);
    })
    .await;
}

// #[actix_rt::test]
// async fn put_todo() {
//     spawn_app(|test_app| async move {
//         insert_todos(&test_app.pool).await;
//         let client = reqwest::Client::new();
//         let response = client
//             .put(&format!(
//                 "{}/api/todos/11111111-1111-1111-1111-111111111111",
//                 &test_app.address
//             ))
//             .body(r#"{"task":"updated task","complete":true}"#)
//             .send()
//             .await
//             .expect("Failed to execute request to get todos");

//         assert!(response.status().is_success());
//         let body = response.text().await.expect("Failed to get body");
//         assert_eq!(
//             body,
//             json!(
//                 {
//                     "id": "11111111-1111-1111-1111-111111111111",
//                     "task": "updated task",
//                     "complete": true
//                 }
//             )
//             .to_string()
//         );
//     })
//     .await;
// }
