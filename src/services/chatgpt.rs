use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

// Definição da struct ChatGPTRequest
#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
}

// Definição da struct Message
#[derive(Serialize, Deserialize)]  // Aqui adicionamos Deserialize
struct Message {
    role: String,
    content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Message {
        Message {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

// Definição da struct ChatGPTResponse
#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

// Método modificado para receber o prompt e devolver a resposta do ChatGPT
pub async fn send_message_to_chatgpt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env
    let api_key = env::var("OPENAI_API_KEY").expect("A variável OPENAI_API_KEY não está definida");

    let client = Client::new();

    // Corpo da requisição para o ChatGPT
    let request_body = ChatGPTRequest {
        model: "gpt-4".to_string(),
        messages: vec![
            Message::new("system", "Você é um assistente útil."),
            Message::new("user", prompt),
        ],
    };

    // Envia a requisição para a API do ChatGPT
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?
        .json::<ChatGPTResponse>()
        .await?;

    // Pega a primeira resposta (se existir) e retorna como string
    let reply = response.choices.first()
        .map(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "Sem resposta".to_string());

    Ok(reply)
}
