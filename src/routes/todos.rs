use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TodoRequest {
    task: String,
    complete: bool,
}

#[derive(Serialize, Debug)]
pub struct Todo {
    id: Uuid,
    task: String,
    complete: bool,
}

#[get("")]
pub async fn get_todos(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(r#"SELECT id, task, complete FROM todo"#)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(todos) => {
            println!("GET todos {:?}", todos);
            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("/{id}")]
pub async fn get_todo(id: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let uuid = match Uuid::parse_str(id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::InternalServerError(),
    };

    match sqlx::query!(
        r#"
        SELECT id, task, complete 
        FROM todo
        WHERE id = $1
        "#,
        uuid
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(rec) => {
            let todo = Todo {
                id: rec.id,
                task: rec.task,
                complete: rec.complete,
            };
            println!("GET todo {:?}", todo);
            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("")]
pub async fn create_todo(todo: web::Json<TodoRequest>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO todo
            (id, task, complete)
        VALUES
            ($1, $2, $3)
        RETURNING id
        "#,
        Uuid::new_v4(),
        todo.task,
        todo.complete
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("")]
pub async fn update_todo() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("")]
pub async fn delete_todo() -> impl Responder {
    HttpResponse::Ok()
}
