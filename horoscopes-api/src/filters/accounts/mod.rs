use warp::Filter;

mod login;
mod signup;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let accounts_prefix = warp::path(prefix);
    accounts_prefix.and(login::filter().or(signup::filter()))
}
