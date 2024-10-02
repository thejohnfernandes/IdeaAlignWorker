mod db; 
mod services;
use crate::services::chatgpt::send_message_to_chatgpt;
use regex::Regex;

struct SQLQuery {
    action: String,
    table: String,
    columns: Vec<String>,
    values: Vec<String>,
}


pub fn identificar_query(query: &str) -> Option<SQLQuery> {
    // Regex para capturar a estrutura básica de um comando SQL do tipo INSERT
    let re = Regex::new(
        r"INSERT INTO (\w+\.\w+) \(([\w, ]+)\) VALUES\(([^;]+)\);"
    ).unwrap();

    // Tenta encontrar uma correspondência com a regex
    if let Some(caps) = re.captures(query) {
        // Extrai a ação, tabela, colunas e valores
        let table = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let columns = caps.get(2).map_or("", |m| m.as_str()).split(", ")
            .map(|s| s.to_string()).collect();
        let values = caps.get(3).map_or("", |m| m.as_str()).split(", ")
            .map(|s| s.trim_matches('\'').to_string()).collect();
        
        // Cria e retorna a struct SQLQuery
        return Some(SQLQuery {
            action: "INSERT".to_string(),
            table,
            columns,
            values,
        });
    }

    // Retorna None se não conseguir identificar a query
    None
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = db::get_db_connection().await?;

    
    // Chame a função get_messages e exiba o resultado
    let messages = db::queries::get_messages(&client).await?;
    
    
    for message in messages {
        let parte = format!("ID: {}, Resposta: {}\n", message.id, message.resposta);
 
        let txt_origine = String::from("agora nos cabe refutar suas palavras, que são: 'Nem um deus, ó judeus e cristãos, nem um filho de Deus jamais desceu ou pode descer ao mundo. Mas se vocês falam de não sei quais anjos, a quem vocês chamam assim, deuses ou algum outro tipo de seres? A outro tipo de seres, ao que parece, aos demônios'. Celso aqui se repete, pois já disse muitas vezes o mesmo anteriormente (IV 2-23), e, portanto, não é necessário discutir extensivamente. Basta o que já dissemos sobre isso. No entanto, argumentaremos algo entre muitas coisas que poderiam ser ditas, que nos parece estar em concordância com o que foi dito antes, embora não seja exatamente o mesmo. Assim demonstraremos que, se Celso realmente sustenta de forma universal que nenhum deus ou filho de Deus jamais desceu aos homens, ele derruba o que as pessoas acreditam sobre a aparição de algum deus e o que ele mesmo disse antes (III 22-25). E é assim que, se Celso realmente diz, como um princípio universal, que nenhum deus ou filho de Deus desceu ou pode descer ao mundo, ele evidentemente derruba a tese de que há deuses na terra, que desceram do céu, seja para dar oráculos sobre o futuro aos homens, seja para curá-los por esses mesmos oráculos. Consequentemente, nem Apolo Pítio, nem Asclépio, nem qualquer outro deus que se acredita fazer tudo isso, seria um deus que desceu do céu; e, se é um deus, teria sido relegado a habitar a terra como uma espécie de fugitivo da mansão dos deuses. Seria como um miserável a quem não é permitido entrar na parte das coisas divinas que lá existem; ou, finalmente, nem Apolo nem Asclépio seriam deuses que se acredita fazerem algo na terra, mas seriam demônios muito inferiores aos homens sábios, que, por sua virtude, ascendem ao firmamento celeste (cf. Platão, Fedro 247b).");
        let prompt = format!("Com base no texto de Orígenes, me retorne esse insert INSERT INTO public.refutacoes_origenes (refutacao, proximidade, justificativa) VALUES('', 0, ''); um numero de 0 a 10  qual é a proximidade da refutação com Orígenes? texto de origines:{} Refutação: {}", txt_origine, parte );
        
    
        let resposta_chatgpt = send_message_to_chatgpt(&prompt).await?;
        
        if let Some(sql_query) =  identificar_query(&resposta_chatgpt) {
            // Imprimir a struct SQLQuery usando Debug
            println!("{:?}", sql_query);
        } else {
            println!("Não foi possível identificar a query.");
        }
    }


 
    Ok(())
}
