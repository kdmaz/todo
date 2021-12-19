use actix_web::{delete, get, post, put, HttpResponse, Responder};

// struct Todo {
//     id: i32,
//     task: String,
//     complete: bool,
// }

#[get("")]
pub async fn get_todos() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/:id")]
pub async fn get_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[post("")]
pub async fn create_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[put("")]
pub async fn update_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("")]
pub async fn delete_todo() -> impl Responder {
    HttpResponse::Ok()
}
