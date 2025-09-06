use reqwest::Client;

// Create a struct to hold application state
pub struct AppState {
    pub http_client: Client,
    //pub jwt_secret: String,
    pub gemini_api_key: String,
}
