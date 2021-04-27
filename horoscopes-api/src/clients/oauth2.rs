use std::env;

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};

const GOOGLE_OAUTH_CLIENT_ID_ENV_KEY: &str = "GOOGLE_OAUTH_CLIENT_ID";
const GOOGLE_OAUTH_CLIENT_SECRET_ENV_KEY: &str =
    "GOOGLE_OAUTH_CLIENT_SECRET";

const GOOGLE_OAUTH_TOKEN_URL: &str =
    "https://accounts.google.com/o/oauth2/token";
const GOOGLE_OAUTH_AUTH_URL: &str =
    "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_OAUTH_REDIRECT_URL: &str =
    "http://localhost:3030/oauth2/callback";

pub fn gen_google_oauth_client() -> BasicClient {
    let google_oauth_client_id =
        env::var(GOOGLE_OAUTH_CLIENT_ID_ENV_KEY).expect(&format!(
            "Failed to fetch env: {}.",
            GOOGLE_OAUTH_CLIENT_ID_ENV_KEY
        ));
    let google_oauth_client_secret = env::var(
        GOOGLE_OAUTH_CLIENT_SECRET_ENV_KEY,
    )
    .expect(&format!(
        "Failed to fetch env: {}.",
        GOOGLE_OAUTH_CLIENT_SECRET_ENV_KEY
    ));
    let token_url = TokenUrl::new(GOOGLE_OAUTH_TOKEN_URL.to_string())
        .expect("Failed to parse GOOGLE_OAUTH_TOKEN_URL");
    let auth_url = AuthUrl::new(GOOGLE_OAUTH_AUTH_URL.to_string())
        .expect("Failed to parse GOOGLE_OAUTH_AUTH_URL");

    let client = BasicClient::new(
        ClientId::new(google_oauth_client_id.to_string()),
        Some(ClientSecret::new(
            google_oauth_client_secret.to_string(),
        )),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new(GOOGLE_OAUTH_REDIRECT_URL.to_string())
            .unwrap(),
    );

    return client;
}
