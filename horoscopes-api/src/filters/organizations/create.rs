use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;
use crate::filters::errors::from_usecase_error;
use crate::filters::{with_auth, with_usecase};
use crate::usecases::Usecase;
use crate::usecases::organizations::create_organization_usecase::{self, CreateOrganizationUsecase};
use crate::adapters::infrastructure::{
    repositories::persistence::organization_repository::OrganizationRepositoryImpl,
    providers::id::ulid_provider::ULIDProvider,
};

pub fn filter(
    app_state: Arc<AppState>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let organization_repository = Arc::new(OrganizationRepositoryImpl::new(app_state.pool())); 
    let ulid_provider = Arc::new(ULIDProvider::new());

    let deps = create_organization_usecase::Deps::new(ulid_provider, organization_repository);
    let usecase = CreateOrganizationUsecase::new(deps);

    warp::path::end()
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    user_id: String,
    body: RequestBody,
    usecase: CreateOrganizationUsecase,
) -> Result<impl warp::Reply, warp::Rejection> {
    let new_organization = create_organization_usecase::NewOrganizationDTO {
        name: body.organization.name,
    };

    let input = create_organization_usecase::Input {
        current_user_id: user_id,
        organization: new_organization,
    };

    let result = usecase.run(input);

    match result {
        Err(err) => {
           let app_error = from_usecase_error(err);
           Err(warp::reject::custom(app_error))
        },
        Ok(output) => {
            let organization_dto = output.organization;
            let organization_res_body = from_dto(organization_dto);
            let res_body = ResponseBody {
               data: organization_res_body,
            };

            Ok(warp::reply::json(&res_body)).map(|rep| {
                warp::reply::with_status(
                rep,
                warp::http::StatusCode::CREATED,
                )
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    organization: NewOrganization,
}

#[derive(Serialize, Deserialize)]
struct NewOrganization {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    data: OrganizationResponseBody
}

#[derive(Serialize, Deserialize)]
struct OrganizationResponseBody {
    id: String,
    name: String,
    user_ids: Vec<String>,
}

fn from_dto(dto: create_organization_usecase::OrganizationDTO) -> OrganizationResponseBody {
    OrganizationResponseBody {
        id: dto.id,
        name: dto.name,
        user_ids: dto.user_ids,
    }
}
