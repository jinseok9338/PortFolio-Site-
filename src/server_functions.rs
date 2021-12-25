use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[path = "todo_functions.rs"] mod server_functions;



// I need to make impl for showing the todos
#[get("/")]
pub async fn createTodos() -> impl Responder {
    
    let mut todosConstruct = server_functions::Todos {
        todos: Vec::new()
    };
    HttpResponse::Ok().body(todosConstruct)
}

pub async  fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}