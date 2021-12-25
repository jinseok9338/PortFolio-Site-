use chrono::prelude::*;


#[derive(Debug)]
struct Todo {
    name:String,
    done: bool,
    when_done:DateTime<Utc>,
    id: u32,
}
#[derive(Debug)]
pub struct Todos {
  todos: Vec<Todo>
}


// Add, Delelte, ToggleDone, 

impl Todos {

   
    fn show_todos(&self) {
        for item in self.todos.iter() {
            println!("{:?}", item)
        }
    }

    pub fn add_todo(&mut self,name:String)  {
        
        let item = Todo {
            name:name,
            done: false,
            when_done: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),  // TODO I need a default datetime for this 
            id: self.todos.len() as u32 + 1
        };

        println!("{:?}", item)

        self.todos.push(item)
    }


 
    pub fn delete_todo(&mut self, id:u32) {
            let index = self.todos.iter().position(|x| x.id == id).unwrap();
            self.todos.remove(index);
            
    }

    pub fn toggle_done(&mut self, id:u32)  {
        let index = self.todos.iter().position(|x| x.id == id).unwrap();
        if self.todos[index].done == false {
            self.todos[index].done = true
        }
        self.todos[index].done = false 
    }

}