pub mod queries;

use tokio_postgres::{Client, NoTls, Error};
use std::env;

// Função para configurar e retornar a conexão com o banco de dados
pub async fn get_db_connection() -> Result<Client, Error> {
    dotenv::dotenv().ok(); // Carregar variáveis de ambiente do arquivo .env
    
    let db_url = env::var("CONNECTION_STRING").expect("A variável de ambiente CONNECTION_STRING não está configurada.");
    
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    
    // Rodar a conexão em background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão: {}", e);
        }
    });
    
    Ok(client)
}
