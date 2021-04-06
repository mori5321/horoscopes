use warp::Filter;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let todos_prefix = warp::path(prefix);
    todos_prefix
        .and(
            list()
                .or(create())
                .or(update())
                .or(destroy())
        )
}

fn list(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::get().map(|| Ok(warp::reply::json(&"List Todo")))
}

fn get(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
+ Clone {
    warp::get()
        .and(warp::path::param())
        .map(|id: u64|
            Ok(
                warp::reply::json(
                    &format!("Show Todo, {}", id.to_string())
                )
            )
        )
}

fn create(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::post()
        .map(|| Ok(warp::reply::json(&"Create Todo")))
}

fn update(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::patch()
        .and(warp::path::param())
        .map(|id: u64| 
            Ok(
                warp::reply::json(
                    &format!("Update Todo, {}", id.to_string())
                )
            )
        )
}

fn destroy(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::delete()
        .and(warp::path::param())
        .map(|id: u64|
            Ok(
                warp::reply::json(
                    &format!("Delete Todo, {}", id.to_string())
                )
            )
        )
}
