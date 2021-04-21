use crate::domain::repositories::TodoRepository;
use crate::domain::entities::todo::Todo;
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
    pub new_todo_dto: NewTodoDTO 
}

pub struct Output {
    pub success: bool
}

pub struct NewTodoDTO {
    pub title: String
}


#[derive(Clone)]
pub struct CreateTodoUsecase {
    deps: Deps
}

impl Usecase<Input, Output, Deps> for CreateTodoUsecase {
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Output {
        // TODO: generate ULID or define TodoFactory which generates ULID.
        // この辺のUserFactoryのアイデアもよい
        // https://github.com/nrslib/BottomUpDDDTheLaterPart/tree/master/src
        let id = "xxxxxx".to_string();
        let res_todo = Todo::new(
            id,
            input.new_todo_dto.title,
            false
        );

        if let Err(_) = res_todo {
            return Output { success: false }
        }

        // TODO: We need better error handling.
        match self.deps.todo_repository.store(res_todo.unwrap()) {
            Ok(_) => {
                Output {
                    success: true
                }
            },
            Err(_) => {
                // TODO: Handle Error
                // format!("Error: {}", msg);
                // Err("Failed to create todo")
                Output {
                    success: false
                }
            }
        }
    }
}
