use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::usecases::Usecase;
use crate::usecases::errors::{UsecaseError, SystemError, UsecaseErrorType};
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
        // TODO: generate ULID or define TodoFactory which generates ULID.
        // この辺のUserFactoryのアイデアもよい
        // https://github.com/nrslib/BottomUpDDDTheLaterPart/tree/master/src
        let id = "xxxxxx".to_string();
        let todo = Todo::new(
            id.clone(),
            input.new_todo_dto.title,
            false
        );


        // TODO: We need better error handling.
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
