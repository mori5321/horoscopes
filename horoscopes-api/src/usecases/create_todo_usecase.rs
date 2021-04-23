use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::domain::services::todo_service;
use crate::usecases::Usecase;
use crate::usecases::common::errors::{
    UsecaseError,
    SystemError,
    UsecaseErrorType,
    BusinessError
};
use crate::usecases::common::ports::providers::IDProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    todo_repository: Arc<dyn TodoRepository>,
    id_provider: Arc<dyn IDProvider>
}

impl Deps {
    pub fn new(
        todo_repository: Arc<dyn TodoRepository>,
        id_provider: Arc<dyn IDProvider>
    ) -> Self {
        Self {
            todo_repository,
            id_provider,
        }
    }
}

pub struct Input {
    pub new_todo_dto: NewTodoDTO 
}

pub struct Output {
    pub id: String
}

pub struct NewTodoDTO {
    pub title: String
}


#[derive(Clone)]
pub struct CreateTodoUsecase {
    deps: Deps
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps> for CreateTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        let id = self.deps.id_provider.generate();

        let todo = Todo::new(
            id.clone(),
            input.new_todo_dto.title,
            false
        );

        let service = todo_service::TodoService::new(self.deps.todo_repository.clone());

        if service.is_duplicated(&todo) {
            return Err(UsecaseError {
                child: None,
                err_type: UsecaseErrorType::BusinessError(BusinessError::DuplicatedError),
                message: "Todo of this ID already exists.".to_string(),
            })
        }
            
        if let Err(e) = validator::validate_todo(&todo) {
            return Err(e)
        }

        match self.deps.todo_repository.store(todo) {
            Ok(_) => {
                Ok(Output {
                    id 
                })
            },
            Err(_) => {
                Err(
                    UsecaseError {
                        child: None,
                        err_type: UsecaseErrorType::SystemError(SystemError::UnknownError),
                        message: "Temporary Error".to_string(),
                    }
                )
            }
        }
    }
}

mod validator {
    use super::*;

    const TITLE_MAX_LENGTH: usize = 80;

    pub fn validate_todo(todo: &Todo) -> Result<(), UsecaseError> {
        if todo.title().value().len() > TITLE_MAX_LENGTH {
            return Err(
                UsecaseError {
                    child: None,
                    err_type: UsecaseErrorType::BusinessError(
                        BusinessError::ValidationError
                    ),
                    message: format!(
                        "Title length must be less than {}.",
                        TITLE_MAX_LENGTH,
                    )
                }
            )
        }
        
        return Ok(())
    }
}
