use std::env;
use tokio_postgres::NoTls;

pub async fn get_db_connection() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    dotenv::dotenv().ok(); // Carregar variáveis de ambiente do .env
    let db_url = env::var("CONNECTION_STRING").expect("CONNECTION_STRING não está configurado.");
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão: {}", e);
        }
    });
    Ok(client)
}
