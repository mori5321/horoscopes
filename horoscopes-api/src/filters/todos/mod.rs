use warp::Filter;

mod list;
mod get;
mod create;
mod update;
mod delete;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let todos_prefix = warp::path(prefix);
    todos_prefix
        .and(
            list::filter()
                .or(get::filter())
                .or(create::filter())
                .or(update::filter())
                .or(delete::filter())
        )
}
