use axum::{
    routing::post,
    Router,
    extract::{Json, State},
    response::IntoResponse
};
use serde::{Deserialize, Serialize}; 
use reqwest::Client;

#[derive(Debug, Deserialize, Serialize)]
struct ChatReq {
    prompt: String,
    model: String,
    #[serde(default)]
    stream: bool
}

//starts multi-threaded system to run code
#[tokio::main]
async fn main() { 
    //create shared http client when app starts
    let http_client = Client::new();

    let app = Router::new()
            .route("/chat", post(handler))
            .with_state(http_client);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Proxy listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    State(client): State<Client>,
    Json(payload): Json<ChatReq>
) -> impl IntoResponse {
    println!("Received request {:?}", payload.model);
    
    //read backend url from environment to remain portable across scenarios
    let ai_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1::11434/api/generate".to_string());
    //fall back to default if variable isn't set

    let response = client
        .post(ai_url)
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            //for now grab raw text and return
            let body = res.text()
                .await
                .unwrap_or_else
                (|_| "Failed to read AI response".to_string());
                body
        },
        Err(e) => {
            eprintln!("Failed to reach local AI: {}", e); //for dev in console
            "Error: Proxy could not reach the local AI endpoint.".to_string() //for user
        }
    }
}