use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod todo_functions;
mod server_functions;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(server_functions::createTodos)
            .route("/", web::get().to(server_functions::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


// let mut todosConstruct = todo_functions::Todos {
//     todos: Vec::new()
// };


// todosConstruct.add_todo("My first Todo");
// todosConstruct.add_todo("My second Todo");

// todosConstruct.delete_todo(2);
// todosConstruct.toggle_done(1);
// todosConstruct.show_todos();
// todosConstruct.toggle_done(1); // This is problem
// todosConstruct.show_todos();