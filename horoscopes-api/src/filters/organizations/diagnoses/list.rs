use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;
use crate::filters::{with_auth, with_usecase};
use crate::filters::errors::from_usecase_error;
use crate::usecases::Usecase;
use crate::usecases::diagnoses::list_diagnoses_usecase::{self, ListDiagnosesUsecase};
use crate::adapters::infrastructure::repositories::persistence::diagnosis_repository::DiagnosisRepositoryImpl;

pub fn filter(
    path: String,
    app_state: Arc<AppState>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let diagnosis_repository = Arc::new(DiagnosisRepositoryImpl::new(app_state.pool()));
    
    let deps = list_diagnoses_usecase::Deps::new(diagnosis_repository);
    let usecase = ListDiagnosesUsecase::new(deps);

    warp::path::param::<String>()
        .and(warp::path::path(path))
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth())
        .and(with_usecase(usecase))
        .and_then(handler)
}

async fn handler(
    organization_id: String,
    user_id: String,
    usecase: ListDiagnosesUsecase,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("OrganizationID: {:?}", organization_id);

    let input = list_diagnoses_usecase::Input {
        current_user_id: user_id,
        organization_id,
    };
    let result = usecase.run(input);

    let output = result.map_err(|err| {
        let app_error = from_usecase_error(err);
        warp::reject::custom(app_error)
    })?;

    let diagnoses_res_body = output.diagnoses.into_iter().map(|diagnosis| {
        from_dto(diagnosis)
    }).collect();
    let res_body = ResponseBody { data: diagnoses_res_body };

    Ok(warp::reply::json(&res_body)).map(|rep| {
        warp::reply::with_status(
            rep,
            warp::http::StatusCode::OK,
        )
    })
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    data: Vec<DiagnosisResponseBody>,
}

#[derive(Serialize, Deserialize)]
struct DiagnosisResponseBody {
    id: String,
    title: String,
    organization_id: String,
}

fn from_dto(dto: list_diagnoses_usecase::DiagnosisDTO) -> DiagnosisResponseBody {
    DiagnosisResponseBody {
        id: dto.id,
        title: dto.title,
        organization_id: dto.organization_id,
    }
}
