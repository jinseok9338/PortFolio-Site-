mod todo_functions;

fn main() {


    let mut todosConstruct = todo_functions::Todos {
        todos: Vec::new()
    };

    
    todosConstruct.add_todo("My first Todo");
    todosConstruct.add_todo("My second Todo");
    
    todosConstruct.delete_todo(2);
    todosConstruct.toggle_done(1);
    todosConstruct.show_todos();
    todosConstruct.toggle_done(1); // This is problem
    todosConstruct.show_todos();
}
