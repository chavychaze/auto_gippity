use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use serde::de::Error;
use std::env;
use tokio::sync::mpsc::error;

use reqwest::header::{HeaderMap, HeaderValue};

// Call large language model (i.e. GPT-3.5)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String =
        env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");
    let ai_agent_version: String = env::var("OPEN_AI_VERSION").expect("OPEN_AI_VERSION not found");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion: ChatCompletion = ChatCompletion {
        model: ai_agent_version, // "gpt-3.5-turbo".to_string(), // "gpt-4o".to_string(), // Paid version
        messages,
        temperature: 0.1,
    };

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(e) => {
                dbg!(e.to_string());
                assert!(false);
            }
        }
    }
}
