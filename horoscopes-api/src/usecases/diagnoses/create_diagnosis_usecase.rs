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
    id_provider: Arc<dyn IDProvider>,
    diagnosis_repository: Arc<dyn DiagnosisRepository>,
}

impl Deps {
    pub fn new(
        id_provider: Arc<dyn IDProvider>,
        diagnosis_repository: Arc<dyn DiagnosisRepository>,
    ) -> Self {
        Self {
            id_provider,
            diagnosis_repository,
        }
    }
}

pub struct Input {
    pub current_user_id: String,
    pub diagnosis: NewDiagnosisDTO,
}

pub struct NewDiagnosisDTO {
    pub title: String,
    pub organization_id: String,
}

pub struct Output {
    pub diagnosis: DiagnosisDTO,
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
pub struct CreateDiagnosisUsecase {
    deps: Deps,
}

impl Usecase<Input, Result<Output, UsecaseError>, Deps>
    for CreateDiagnosisUsecase
{
    fn new(deps: Deps) -> Self {
        Self { deps }
    }
    fn run(&self, input: Input) -> Result<Output, UsecaseError> {
        let diagnosis_id = self.deps.id_provider.generate();

        let new_diagnosis = Diagnosis::new(
            diagnosis_id,
            input.diagnosis.title,
            input.diagnosis.organization_id,
        );

        let res_diagnosis =
            self.deps.diagnosis_repository.store(&new_diagnosis);

        match res_diagnosis {
            Err(err) => {
                let error = UsecaseError::new(
                    UsecaseErrorType::SystemError(
                        SystemError::UnknownError,
                    ),
                    err,
                );
                Err(error)
            }
            Ok(_) => {
                let diagnosis_dto = from_entity(&new_diagnosis);
                Ok(Output {
                    diagnosis: diagnosis_dto,
                })
            }
        }
    }
}
