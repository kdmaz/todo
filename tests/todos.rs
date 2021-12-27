mod utils;
use utils::spawn_app;

#[actix_rt::test]
async fn get_todos() {
    spawn_app(|test_app| async move {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/todos", &test_app.address))
            .send()
            .await
            .expect("Failed to execute request to get todos");

        assert!(response.status().is_success());
    })
    .await;
}
