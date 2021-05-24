use crate::domain::entities::diagnosis::Diagnosis;
use crate::domain::repositories::{
    DiagnosisRepository, OrganizationRepository,
};

use crate::usecases::common::errors::{
    BusinessError, SystemError, UsecaseError, UsecaseErrorType,
};
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::Usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    diagnosis_repository: Arc<dyn DiagnosisRepository>,
}

impl Deps {
    pub fn new(
        diagnosis_repository: Arc<dyn DiagnosisRepository>,
    ) -> Self {
        Self {
            diagnosis_repository,
        }
    }
}

pub struct Input {
    pub current_user_id: String,
    pub organization_id: String,
}

pub struct Output {
    pub diagnoses: Vec<DiagnosisDTO>,
}

pub struct DiagnosisDTO {
    pub id: String,
    pub title: String,
    pub organization_id: String,
}

fn from_entity(diagnosis: &Diagnosis) -> DiagnosisDTO {
    DiagnosisDTO {
        id: diagnosis.id().value(),
        title: diagnosis.title().value(),
        organization_id: diagnosis.organization_id().value(),
    }
}

#[derive(Clone)]
pub struct ListDiagnosesUsecase {
    deps: Deps,
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps>
    for ListDiagnosesUsecase
{
    fn new(deps: Deps) -> Self {
        Self { deps }
    }

    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        // TODO: 指定したOrganizationのDiagnosisのみを返却する。
        // TODO: UserがOrganizationに所属しているかCheckする。
        let diagnoses = self.deps.diagnosis_repository.list();

        let diagnoses_dto = diagnoses
            .into_iter()
            .map(|diagnosis| from_entity(&diagnosis))
            .collect();

        Ok(Output {
            diagnoses: diagnoses_dto,
        })
    }
}
