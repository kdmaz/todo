use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

// struct Todo {
//     id: i32,
//     task: String,
//     complete: bool,
// }

#[get("")]
async fn get_todos() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/:id")]
async fn get_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[post("")]
async fn create_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[put("")]
async fn update_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("")]
async fn delete_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[get("")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let todos_scope = web::scope("/todos")
            .service(get_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo);

        let health_check_scope = web::scope("/health_check").service(health_check);

        let app_scope = web::scope("/api")
            .service(todos_scope)
            .service(health_check_scope);

        App::new().service(app_scope)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
