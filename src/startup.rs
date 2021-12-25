use std::net::TcpListener;

use crate::routes::{create_todo, delete_todo, get_todo, get_todos, health_check, update_todo};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        let health_check_scope = web::scope("/health_check").service(health_check);

        let todos_scope = web::scope("/todos")
            .service(get_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo);

        let app_scope = web::scope("/api")
            .service(todos_scope)
            .service(health_check_scope);

        App::new().service(app_scope).app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
