use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::future::Future;
use std::net::TcpListener;
use todo::configuration::{get_configuration, DatabaseConfig};
use todo::startup_todo_api;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

pub async fn spawn_app<Fut>(test_fn: fn(TestApp) -> Fut)
where
    Fut: Future<Output = ()>,
{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Error getting configuration");
    configuration.database.db_name = Uuid::new_v4().to_string();
    let pool = configure_database(&configuration.database).await;

    let server = startup_todo_api(listener, pool.clone()).expect("Failed to startup todo api");
    tokio::spawn(server);

    test_fn(TestApp {
        address,
        pool: pool.clone(),
    })
    .await;

    cleanup(&configuration.database, pool).await;
}

async fn configure_database(db_config: &DatabaseConfig) -> PgPool {
    let connection_string_without_db = db_config.connection_string_without_db();
    let mut connection = PgConnection::connect(&connection_string_without_db)
        .await
        .expect("Failed to connect to postgres (Create new UUID db)");
    connection
        .execute(format!("CREATE DATABASE \"{}\";", db_config.db_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_string = db_config.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres (Pool)");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}

async fn cleanup(db_config: &DatabaseConfig, pool: PgPool) {
    pool.close().await;

    let mut connection = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to postgres (Cleanup)");
    connection
        .execute(format!("DROP DATABASE \"{}\";", &db_config.db_name).as_str())
        .await
        .expect(format!("Failed to drop database {}", db_config.db_name).as_str());
}

pub async fn insert_todos(pool: &PgPool) {
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
