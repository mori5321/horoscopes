use crate::domain::entities::todo;
use crate::domain::entities::todo::Todo;
use crate::domain::repositories::TodoRepository;

use once_cell::sync::Lazy;
use std::sync::Mutex;


pub struct TodoRepositoryOnMemory {}

static TODOS_ON_MEMORY: Lazy<Mutex<Vec<Todo>>> = Lazy::new(|| {
    let todos = vec![
        Todo::new("a".to_string(), "hello".to_string(), false),
        Todo::new("b".to_string(), "world".to_string(), false),
        Todo::new("c".to_string(), "Let's Sing!".to_string(), false),
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
        // TODO: 存在チェックしてUpdateと切り替える。
        TODOS_ON_MEMORY.lock().unwrap().push(todo.clone());
        Ok(())
    }
}
