//axum is the engine handling HTTP requests, importing routing::post allows proxy to accept data
//extract::Json looks for JSON data and converts into usable format upon arrival of request
use axum::{routing::post, Router, extract::Json, response::IntoResponse};
//Serde is a Serializer/Deserializer

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ChatReq {
    prompt: String,
    model: String,
}

//concurrency in Rust is handled using an executor (tokio)
//starts multi-threaded system to run code
#[tokio::main]

//tells rust that the function will wait for network activity
//and wont freeze computer waiting for request
async fn main() { 
    let app = Router::new().route("/chat", post(handler)); // sends post request @ /chat to the handler


    //
    let listener = tokio::net::TcpListener::bind("127.0.0:3000").await.unwrap();
    println!("Proxy listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Json(payload): Json<ChatReq>) -> impl IntoResponse {
    println!("Received request {:?}", payload);
    "Request Received" //returns this
}
