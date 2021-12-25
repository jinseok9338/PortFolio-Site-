use chrono::prelude::*;


#[derive(Debug)]
pub struct Todo {
    pub name:String,
    pub done: bool,
    pub when_done:DateTime<Utc>,
    pub id: u32,
}
#[derive(Debug)]
pub struct Todos {
  pub todos: Vec<Todo>
}


// Add, Delelte, ToggleDone, 

impl Todos {

   
    pub fn show_todos(&self) {
        for item in self.todos.iter() {
            println!("{:?}", item)
        }
    }

    pub fn add_todo(&mut self,name:&str)  {
        
        let item = Todo {
            name:name.to_string(),
            done: false,
            when_done: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),  
            id: self.todos.len() as u32 + 1
        };

        self.todos.push(item)
    }


 
    pub fn delete_todo(&mut self, id:u32) {
            let index = self.todos.iter().position(|x| x.id == id).unwrap();
            self.todos.remove(index);      
    }

    pub fn toggle_done(&mut self, id:u32)  {
        let index = self.todos.iter().position(|x| x.id == id).unwrap();
        if self.todos[index].done == false {
            
            self.todos[index].done = true;
            println!("{}",self.todos[index].done);
            self.todos[index].when_done = Utc::now();

        }
        else{
            self.todos[index].done = false;
            self.todos[index].when_done = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
        }
        
    }

}