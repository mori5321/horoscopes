use crate::usecases::common::errors::UsecaseError;
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::Usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    id_provider: Arc<dyn IDProvider>,
}

impl Deps {
    pub fn new(id_provider: Arc<dyn IDProvider>) -> Self {
        Self { id_provider }
    }
}

pub struct Input {
    pub name: String,
}

pub struct Output {
    // organization
}

#[derive(Clone)]
pub struct CreateOrganizationUsecase {
    deps: Deps,
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps>
    for CreateOrganizationUsecase
{
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        Ok(Output {})
    }
}
