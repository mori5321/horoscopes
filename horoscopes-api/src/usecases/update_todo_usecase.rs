use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::domain::entities::todo;
use crate::usecases::Usecase;
use crate::usecases::errors::{
    UsecaseError,
    UsecaseErrorType,
    BusinessError,
    SystemError,
    from_domain_error,
};
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

impl Usecase<Input, Result<Output, UsecaseError>, Deps> for UpdateTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        let todo_id = todo::ID::from_str(&input.update_todo_dto.id);
        let todo = self.deps.todo_repository.find(todo_id);

        if todo.is_none() {
            return Err(
                UsecaseError {
                    err_type: UsecaseErrorType::BusinessError(BusinessError::NotFoundError),
                    message: "temporary".to_string(),
                    child: None,
                }
            )
        }
        
        let res_updated_todo = Todo::new(
            input.update_todo_dto.id,
            input.update_todo_dto.title,
            input.update_todo_dto.is_done,
        );

        if let Err(err) = res_updated_todo {
            return Err(from_domain_error(err))
        }

        let result = self.deps.todo_repository.store(res_updated_todo.unwrap());
        match result {
            // TODO: we need better error handling.
            Ok(_) => Ok(Output {}),
            Err(err) => {
                return Err(
                    UsecaseError {
                        err_type: UsecaseErrorType::SystemError(SystemError::UnknownError),
                        message: "UnknownError".to_string(),
                        child: None,
                    }
            )
            }
        }
    }
}
