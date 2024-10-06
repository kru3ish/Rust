use serde::{Deserialize, Serialize};
use warp::Filter;

# [tokio::main]
async fn main() {
    // Define the GET /hello route
    let hello = warp::path!("hello")
        .map(|| {
            let response = ResponseMessage {
                message: "Hello, World!".to_string(),
            };
            warp::reply::json(&response)
        });

    // Define the POST /echo route
    let echo = warp::path!("echo")
        .and(warp::body::json())
        .map(|input: InputMessage| {
            warp::reply::json(&input)
        });

    // Combine routes
    let routes = warp::get().and(hello)
        .or(warp::post().and(echo));

    // Start the warp server
    println!("Starting server on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Struct for input JSON (for POST requests)
#[derive(Serialize, Deserialize)]
struct InputMessage {
    message: String,
}

// Struct for response JSON (for GET requests)
#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    message: String,
}