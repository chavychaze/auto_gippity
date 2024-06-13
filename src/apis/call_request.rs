use crate::models::general::llm::{ Message };
use dotenv::dotenv;
use reqwest::Client;
use std::env;

// Call large language model (i.e. GPT-3.5)
pub async fn call_gpt(message: Vec<Message>) {
    dotenv().ok();

    // Extract API Key information
    let api_key: String = env
        ::var("OPEN_AI_KEY")
        .expect("OPEN_AI_KEY not found in environment variables");
    let api_org: String = env
        ::var("OPEN_AI_ORG")
        .expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";
}
