use warp::Filter;

mod create;
mod delete;
mod get;
mod list;
mod update;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let todos_prefix = warp::path(prefix);
    todos_prefix.and(
        list::filter()
            .or(get::filter())
            .or(create::filter())
            .or(update::filter())
            .or(delete::filter()),
    )
}
