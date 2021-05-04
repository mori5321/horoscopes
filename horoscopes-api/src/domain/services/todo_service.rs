use crate::domain::entities::todo::Todo;
use crate::domain::repositories::TodoRepository;
use std::sync::Arc;

pub struct TodoService {
    todo_repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub fn is_duplicated(&self, todo: &Todo) -> bool {
        let opt_todo = self.todo_repository.find(todo.id());
        opt_todo.is_some()
    }
}
