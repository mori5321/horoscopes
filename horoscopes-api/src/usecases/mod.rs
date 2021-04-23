pub mod common;

pub mod list_todos_usecase;
pub mod get_todo_usecase;
pub mod create_todo_usecase;
pub mod update_todo_usecase;
pub mod delete_todo_usecase;

pub trait Usecase<Input, Output, Deps: ?Sized> {
    fn new(deps: Deps) -> Self;
    fn run(&self, input: Input) -> Output;
}

