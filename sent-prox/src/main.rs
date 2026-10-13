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
    

    //call python sidecar script first
    let side_url = "http://localhost:8000/inspect";

    let inspect_payload = serde_json::json!({
        "prompt": payload.prompt
    });

    match client.post(side_url).json(&inspect_payload).send().await {
        Ok(res) => {
            if let Ok(verdict) = res.json::<serde_json::Value>().await {
                let is_safe = verdict["safe"].as_bool().unwrap_or(true);
                if !is_safe {
                    let reason = verdict["reason"].as_str().unwrap_or("Unknown threat");
                    println!("[SECURITY BLOCK] Python sidecar blocked request: {}", reason);

                    return (
                        axum::http::StatusCode::FORBIDDEN,
                        format!("Request blocked by security filter: {}", reason),
                    ).into_response();
                }
            }
        }

        Err(e) => {
            
            eprintln!("Warning: Could not reach python sidecar: {}. Rejecting prompt.", e);
                return (
                    axum::http::StatusCode::FORBIDDEN,
                    format!("Error: Security guardrail is offline. Request blocked for safety."),
                ).into_response();
        }
    }



    //read backend url from environment to remain portable across scenarios
    let ai_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:11434/api/generate".to_string());
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
                body.into_response()
        },
        Err(e) => {
            eprintln!("Failed to reach local AI: {}", e); //for dev in console
            "Error: Proxy could not reach the local AI endpoint.".into_response() //for user
        }
    }
}