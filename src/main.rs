use todo::startup_todo_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    startup_todo_api()?.await
}
