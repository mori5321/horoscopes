pub mod common;

pub mod todos;

pub trait Usecase<Input, Output, Deps: ?Sized> {
    fn new(deps: Deps) -> Self;
    fn run(&self, input: Input) -> Output;
}

