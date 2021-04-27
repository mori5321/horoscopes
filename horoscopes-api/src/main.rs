use horoscopes_api::filters;

use std::env;
use warp::Filter;

#[tokio::main()]
async fn main() {
    let port: u16 = 3030;
    println!("Server is running on port: {}", port);
    
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "horoscopes=debug");
    }
    pretty_env_logger::init();

    warp::serve(
        filters::filters().with(warp::log("horoscopes"))
    ).run(([0, 0, 0, 0], port))
        .await;
}

