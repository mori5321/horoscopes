use crate::clients::oauth2::gen_google_oauth_client;
use oauth2::{
    // AuthorizationCode,
    CsrfToken,
    PkceCodeChallenge,
    Scope,
    // TokenResponse,
};
use serde::Deserialize;
use warp::Filter;
// use oauth2::reqwest::http_client;

pub fn filters(
    prefix: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    let oauth2_prefix = warp::path(prefix);
    oauth2_prefix.and(auth().or(callback()))
}

fn auth(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::path("auth").and(warp::get()).and_then(auth_handler)
}

fn callback(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection>
       + Clone {
    warp::path("callback")
        .and(warp::get())
        .and(warp::query::<CallbackQueries>())
        .and_then(callback_handler)
}

// REF
// https://dev.classmethod.jp/articles/oauth-2-0-pkce-by-auth0/
// https://docs.rs/oauth2/3.0.0/oauth2/
// https://qiita.com/TakahikoKawasaki/items/00f333c72ed96c4da659#%E3%82%B5%E3%83%BC%E3%83%90%E3%83%BC%E5%81%B4%E5%AE%9F%E8%A3%85%E3%81%A7%E3%82%84%E3%82%8B%E3%81%93%E3%81%A8

async fn auth_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let client = gen_google_oauth_client();

    let (pkce_challenge, _pkce_verifier) =
        PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile"
                .to_string(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    // respond_with_json(Ok("auth"), warp::http::StatusCode::MOVED_PERMANENTLY)
    //     .map(|rep| warp::reply::with_header(rep, "Location", auth_url.to_string()))

    Ok(warp::reply::json(&"auth"))
        .map(|rep| {
            warp::reply::with_status(
                rep,
                warp::http::StatusCode::MOVED_PERMANENTLY,
            )
        })
        .map(|rep| {
            warp::reply::with_header(
                rep,
                "Location",
                auth_url.to_string(),
            )
        })
}

#[derive(Deserialize)]
pub struct CallbackQueries {
    code: String,
    state: String,
}

async fn callback_handler(
    q: CallbackQueries,
) -> Result<impl warp::Reply, warp::Rejection> {
    // pkce検証とかいうことをしないといけない。
    // In memory KVSだと複数サーバーにできないので、サーバー外のDBへの保存が必須な気がする。
    println!("{}", q.code);
    println!("{}", q.state);

    Ok(warp::reply::json(&"callback")).map(|rep| {
        warp::reply::with_status(rep, warp::http::StatusCode::OK)
    })
}
