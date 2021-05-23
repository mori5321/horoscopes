use horoscopes_api::filters;
use horoscopes_api::state::AppState;
use horoscopes_api::db::gen_mysql_pool;

use std::env;
use warp::Filter;
use std::sync::Arc;

#[tokio::main()]
async fn main() {
    let port: u16 = 3030;
    println!("Server is running on port: {}", port);

    let pool = Arc::new(gen_mysql_pool()); 
    let app_state = Arc::new(AppState::new(pool.clone()));

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "horoscopes=debug");
    }
    pretty_env_logger::init();

    warp::serve(filters::filters(app_state.clone()).with(warp::log("horoscopes")))
        .run(([0, 0, 0, 0], port))
        .await;
}
