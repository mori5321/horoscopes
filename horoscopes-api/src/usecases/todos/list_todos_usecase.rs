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

pub struct Input {}
pub struct Output {
    pub todos: TodosDTO,
}

pub type TodosDTO = Vec<TodoDTO>;
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
pub struct ListTodosUsecase {
    deps: Deps,
}

impl Usecase<Input, Output, Deps> for ListTodosUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, _input: Input) -> Output {
        let todos = self.deps.todo_repository.list();

        let todos_dto = todos.into_iter().map(from_entity).collect();

        return Output { todos: todos_dto };
    }
}
