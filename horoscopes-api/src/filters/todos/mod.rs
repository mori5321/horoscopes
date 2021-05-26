use warp::Filter;

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn filters(
    path: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let todos_path = warp::path(path);
    todos_path.and(
        list::filter()
            .or(get::filter())
            .or(create::filter())
            .or(update::filter())
            .or(delete::filter()),
    )
}
