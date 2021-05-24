pub mod common;

pub mod accounts;
pub mod diagnoses;
pub mod organizations;
pub mod todos;

pub trait Usecase<Input, Output, Deps: ?Sized> {
    fn new(deps: Deps) -> Self;
    fn run(&self, input: Input) -> Output;
}
