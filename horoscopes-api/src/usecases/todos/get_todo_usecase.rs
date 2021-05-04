use crate::domain::entities::todo;
use crate::domain::entities::todo::Todo;
use crate::domain::repositories::TodoRepository;
use crate::usecases::Usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    todo_repository: Arc<dyn TodoRepository>,
}

impl Deps {
    pub fn new(todo_repository: Arc<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }
}

pub struct Input {
    pub id: String,
}
pub struct Output {
    pub todo: Option<TodoDTO>,
}

pub struct TodoDTO {
    pub id: String,
    pub title: String,
    pub is_done: bool,
}

fn from_entity(todo: Todo) -> TodoDTO {
    TodoDTO {
        id: todo.id().value(),
        title: todo.title().value(),
        is_done: todo.is_done().value(),
    }
}

#[derive(Clone)]
pub struct GetTodoUsecase {
    deps: Deps,
}

impl Usecase<Input, Output, Deps> for GetTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Output {
        let id = todo::ID::from_str(&input.id);
        let todo = self.deps.todo_repository.find(id);

        Output {
            todo: todo.map(|t| from_entity(t)),
        }
    }
}
