use tokio_postgres::Client;

pub struct Mensagem {
    pub id: i32,
    pub resposta: String,
}


// Função para buscar mensagens do banco de dados
pub async fn get_messages(client: &Client) -> Result<Vec<Mensagem>, tokio_postgres::Error> {
    // Consulta SQL para buscar dados da tabela public.respostas_celso
    let rows = client.query("SELECT * FROM public.respostas_celso", &[]).await?;

    let mut messages = Vec::new();

    // Iterar sobre as linhas retornadas e criar uma lista de Mensagem
    for row in rows {
        messages.push(Mensagem {
            id: row.get(0),
            resposta: row.get(1),
        });
    }

    Ok(messages)
}
