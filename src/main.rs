use sqlx::PgPool;
use todo::startup_todo_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/todo_db")
        .await
        .expect("Could not connect to postgres.");

    startup_todo_api(pool)?.await
}
