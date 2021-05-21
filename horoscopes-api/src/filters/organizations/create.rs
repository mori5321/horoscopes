use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

use crate::state::AppState;
use crate::filters::with_auth;

pub fn filter(
    app_state: Arc<AppState>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(with_auth())
        .and(warp::body::json())
        .and_then(handler)
}

async fn handler(
    user_id: String,
    body: RequestBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res_organization = OrganizationResponseBody {
        id: "org_001".to_string(),
        name: body.organization.name.to_string(),
    };

    let res_body = ResponseBody {
        data: res_organization,    
    };

    Ok(warp::reply::json(&res_body)).map(|rep| {
        warp::reply::with_status(
            rep,
            warp::http::StatusCode::CREATED,
        )
    })
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
}
