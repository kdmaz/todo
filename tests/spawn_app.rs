use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use todo::configuration::{get_configuration, DatabaseSettings};
use todo::startup_todo_api;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

// make test_fn async so can be awaited
pub async fn spawn_app(test_fn: fn(TestApp) -> ()) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1/{}", port);

    let mut settings = get_configuration().expect("Error getting settings");
    settings.database.db_name = Uuid::new_v4().to_string();
    let pool = configure_database(&settings.database).await;

    let server = startup_todo_api(listener, pool.clone()).expect("Failed to startup todo api");
    tokio::spawn(server);

    test_fn(TestApp { address, pool }).await;

    cleanup(&settings.database).await;
}

async fn configure_database(db_settings: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&db_settings.connection_string_without_db())
        .await
        .expect("Failed to connect to postgres");
    connection.execute(format!("DATABASE CREATE \"{}\";", db_settings.db_name).as_str());

    let pool = PgPool::connect(&db_settings.connection_string())
        .await
        .expect("Failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}

async fn cleanup(db_settings: &DatabaseSettings) {
    let mut connection = PgConnection::connect(&db_settings.connection_string_without_db())
        .await
        .expect("Failed to connect to postgres");
    connection
        .execute(format!("DROP DATABASE, \"{}\"", &db_settings.db_name).as_str())
        .await
        .expect(format!("Failed to drop database {}", db_settings.db_name).as_str());
}
