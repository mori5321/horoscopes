use horoscopes_api::filters;

#[tokio::main()]
async fn main() {
    let port: u16 = 3030;
    println!("Server is running on port: {}", port);

    warp::serve(filters::filters())
        .run(([127, 0, 0, 1], port))
        .await;
}

