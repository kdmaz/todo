use sqlx::PgPool;
use std::net::TcpListener;
use todo::{configuration::get_configuration, startup_todo_api};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration");

    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    startup_todo_api(listener, pool)?.await
}
