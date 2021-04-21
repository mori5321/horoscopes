use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::domain::entities::todo;
use crate::usecases::Usecase;
use std::sync::Arc;
use std::error::Error;

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

pub struct Output {}

pub struct UpdateTodoDTO {
    pub id: String,
    pub title: String,
    pub is_done: bool
}

#[derive(Clone)]
pub struct UpdateTodoUsecase {
    deps: Deps
}

impl Usecase<Input, Result<Output, String>, Deps> for UpdateTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, String> {
        let todo_id = todo::ID::from_str(&input.update_todo_dto.id);
        let todo = self.deps.todo_repository.find(todo_id);

        if todo.is_none() {
            // return Output { success: false }
            return Err("Not Found Error".to_string())
        }
        
        let res_updated_todo = Todo::new(
            input.update_todo_dto.id,
            input.update_todo_dto.title,
            input.update_todo_dto.is_done,
        );

        if let Err(err) = res_updated_todo {
            println!("Err: {:?}", err.source());
            return Err(err.to_string());
        }

        let result = self.deps.todo_repository.store(res_updated_todo.unwrap());
        match result {
            // TODO: we need better error handling.
            Ok(_) => Ok(Output {}),
            Err(err) => {
                Err(err)        
            }
        }
    }
}
