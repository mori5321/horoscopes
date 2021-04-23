use crate::domain::repositories::TodoRepository;
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
    pub id: String 
}

pub struct Output {
    pub success: bool
}

#[derive(Clone)]
pub struct DeleteTodoUsecase {
    deps: Deps
}

impl Usecase<Input, Output, Deps> for DeleteTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Output {
        let todo_id = todo::ID::from_str(&input.id);
        let result = self.deps.todo_repository.remove(todo_id);
        match result {
            Ok(()) => Output { success: true },
            Err(_msg) => Output { success: false }
        }
    }
}
