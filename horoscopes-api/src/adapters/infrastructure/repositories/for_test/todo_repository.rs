use crate::domain::entities::todo;
use crate::domain::entities::todo::Todo;
use crate::domain::repositories::TodoRepository;

use std::sync::Mutex;

// RefCellかMutexでつくれる。
// 内部可変性。
// https://doc.rust-jp.rs/book-ja/ch15-05-interior-mutability.html
//

pub struct TodoRepositoryForTest {
    pub todos: Mutex<Vec<Todo>>
}

impl TodoRepositoryForTest {
    pub fn new(todos: Vec<Todo>) -> Self {
        Self {
            todos: Mutex::new(todos)
        }
    }

    // pub fn todos(&self) -> Vec<Todo> {
    //     self.todos.lock().unwrap().clone()
    // }
}

impl TodoRepository for TodoRepositoryForTest {
    fn list(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap().clone();
        todos
    }

    fn find(&self, id: todo::ID) -> Option<Todo> {
        let todos = self.todos.lock().unwrap().clone();
        todos.into_iter().find(|todo| todo.id == id)
    }

    fn store(&self, todo: Todo) -> Result<(), String> {
        let mut todos = self.todos.lock().unwrap();

        let opt_idx = todos.clone().into_iter().position(|t| {
            t == todo
        });

        match opt_idx {
            Some(idx) => {
                todos.splice(idx..idx + 1, vec![todo].iter().cloned());
                Ok(())
            },
            None => {
                todos.push(todo.clone());
                Ok(())
            }
        }
    }

    fn remove(&self, id: todo::ID) -> Result<(), String> {
        let mut todos = self.todos.lock().unwrap();

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
