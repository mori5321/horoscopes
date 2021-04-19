use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::domain::entities::todo;
use crate::usecases::Usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    todo_repository: Arc<dyn TodoRepository>
}

impl Deps {
    pub fn new(
        todo_repository: Arc<dyn TodoRepository>
    ) -> Self {
        Self {
            todo_repository
        }
    }
}

pub struct Input {
    pub update_todo_dto: UpdateTodoDTO
}

pub struct Output {
    pub success: bool
}

pub struct UpdateTodoDTO {
    pub id: String,
    pub title: String,
    pub is_done: bool
}

#[derive(Clone)]
pub struct UpdateTodoUsecase {
    deps: Deps
}

impl Usecase<Input, Output, Deps> for UpdateTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Output {
        let todo_id = todo::ID::from_str(&input.update_todo_dto.id);
        let todo = self.deps.todo_repository.find(todo_id);

        if todo.is_none() {
            return Output { success: false }
        }
        
        let updated_todo = Todo::new(
            input.update_todo_dto.id,
            input.update_todo_dto.title,
            input.update_todo_dto.is_done,
        );

        let result = self.deps.todo_repository.store(updated_todo);
        match result {
            // TODO: we need better error handling.
            Ok(_) => Output { success: true },
            Err(_) => Output { success: false }
        }
    }
}
