use crate::domain::entities::organization::Organization;
use crate::domain::repositories::OrganizationRepository;
use crate::usecases::common::errors::{
    SystemError, UsecaseError, UsecaseErrorType,
};
use crate::usecases::common::ports::providers::IDProvider;
use crate::usecases::Usecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct Deps {
    id_provider: Arc<dyn IDProvider>,
    organization_repository: Arc<dyn OrganizationRepository>,
}

impl Deps {
    pub fn new(
        id_provider: Arc<dyn IDProvider>,
        organization_repository: Arc<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            id_provider,
            organization_repository,
        }
    }
}

pub struct Input {
    pub current_user_id: String,
    pub organization: NewOrganizationDTO,
}

pub struct NewOrganizationDTO {
    pub name: String,
}

pub struct Output {
    pub organization: OrganizationDTO,
}

pub struct OrganizationDTO {
    pub id: String,
    pub name: String,
    pub user_ids: Vec<String>,
}

fn from_entity(organization: &Organization) -> OrganizationDTO {
    OrganizationDTO {
        id: organization.id().value(),
        name: organization.name().value(),
        user_ids: organization
            .user_ids()
            .into_iter()
            .map(|user_id| user_id.value())
            .collect(),
    }
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
        let organization_id = self.deps.id_provider.generate();

        let organization = Organization::new(
            organization_id,
            input.organization.name,
            vec![input.current_user_id],
        );

        let res_organization =
            self.deps.organization_repository.store(&organization);

        match res_organization {
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
                let organization_dto = from_entity(&organization);
                Ok(Output {
                    organization: organization_dto,
                })
            }
        }
    }
}
