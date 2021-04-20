use crate::domain::entities::todo;
use crate::domain::entities::todo::Todo;
use crate::domain::repositories::TodoRepository;

use once_cell::sync::Lazy;
use std::sync::Mutex;


pub struct TodoRepositoryOnMemory {}


pub static TODOS_ON_MEMORY: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| {
    let todos = vec![
        Todo::new("ulid-00000001".to_string(), "hello".to_string(), false),
        Todo::new("ulid-00000002".to_string(), "world".to_string(), false),
        Todo::new("ulid-00000003".to_string(), "Let's Sing!".to_string(), false),
    ];
    
    Mutex::new(todos)
});



impl TodoRepositoryOnMemory {
    pub fn new() -> Self {
        Self {}
    } 
}

impl TodoRepository for TodoRepositoryOnMemory {
    fn list(&self) -> Vec<Todo> {
        let todos = TODOS_ON_MEMORY.lock().unwrap().clone();
        todos
    }

    fn find(&self, id: todo::ID) -> Option<Todo> {
        let todos = TODOS_ON_MEMORY.lock().unwrap().clone();
        todos.into_iter().find(|todo| todo.id == id)
    }

    fn store(&self, todo: Todo) -> Result<(), String> {
        let mut todos = TODOS_ON_MEMORY.lock().unwrap();

        let opt_idx = todos.clone().into_iter().position(|t| {
            t == todo
        });

        match opt_idx {
            Some(idx) => {
                todos.splice(idx..idx + 1, vec![todo].iter().cloned());
                Ok(()) 
            }
            None => {
                todos.push(todo.clone());
                Ok(())
            }
        }
    }

    fn remove(&self, id: todo::ID) -> Result<(), String> {
        let mut todos = TODOS_ON_MEMORY.lock().unwrap();

        let opt_idx = todos.clone().into_iter().position(|t| {
            t.id == id
        });

        match opt_idx {
            Some(idx) => {
                todos.remove(idx);
                Ok(())
            },
            None => {
                Err("Not Found Error".to_string())
            }
        }
    }
}
