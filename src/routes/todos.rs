use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TodoRequest {
    task: String,
    complete: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Todo {
    pub id: Uuid,
    pub task: String,
    pub complete: bool,
}

#[get("")]
pub async fn get_todos(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(Todo, "SELECT id, task, complete FROM todo")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
pub async fn get_todo(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        Todo,
        "
            SELECT id, task, complete 
            FROM todo
            WHERE id = $1
        ",
        id.into_inner()
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[post("")]
pub async fn create_todo(todo: web::Json<TodoRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        Todo,
        r#"
            INSERT INTO todo (id, task, complete)
            VALUES ($1, $2, $3)
            RETURNING id, task, complete
        "#,
        Uuid::new_v4(),
        todo.task,
        todo.complete
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
pub async fn update_todo(
    id: web::Path<Uuid>,
    todo: web::Json<TodoRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query_as!(
        Todo,
        r#"
            UPDATE todo
            SET task = $1, complete = $2
            WHERE id = $3
            RETURNING id, task, complete
        "#,
        todo.task,
        todo.complete,
        id.into_inner()
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

#[delete("/{id}")]
pub async fn delete_todo(id: web::Path<Uuid>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(
        Todo,
        r#"
            DELETE FROM todo
            WHERE id = $1
            RETURNING id, task, complete
        "#,
        id.into_inner()
    )
    .fetch_one(pool.as_ref())
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
