# ThoughtProximity - Determining Proximity of Refutations with Origen's Thought Using ChatGPT

This project is a Rust-based application that evaluates the proximity of a given refutation to the thought of Origen using ChatGPT. It analyzes the structure of SQL `INSERT` queries and interacts with a PostgreSQL database to store refutations. The application processes a set of refutations and asks ChatGPT to determine their proximity to Origen's writings on a scale of 0 to 10.

## Features

- **Regex-based SQL Query Identification**: Extracts SQL commands from ChatGPT's responses using regular expressions.
- **Interaction with ChatGPT**: Sends prompts to ChatGPT and processes the response to determine the similarity between a refutation and Origen's thought.
- **PostgreSQL Database Integration**: Queries and inserts refutations and their proximity to Origen's thought into the database.

## Dependencies

- [Tokio](https://tokio.rs/): For asynchronous runtime in Rust.
- [Regex](https://docs.rs/regex/): For pattern matching in the SQL query string.
- [ChatGPT API](https://beta.openai.com/docs/api-reference/): Integration for querying ChatGPT to evaluate the refutations.
- PostgreSQL: The project uses a PostgreSQL database to store refutations.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/thought-proximity.git
    cd thought-proximity
    ```

2. Add the necessary dependencies in `Cargo.toml`:
    ```toml
    [dependencies]
    tokio = { version = "1", features = ["full"] }
    regex = "1"
    reqwest = { version = "0.11", features = ["json"] }
    serde = { version = "1", features = ["derive"] }
    serde_json = "1"
    ```

3. Setup the PostgreSQL database:
    - Ensure you have a running instance of PostgreSQL.
    - Create a table named `refutacoes_origenes` in your database:
      ```sql
      CREATE TABLE public.refutacoes_origenes (
          id SERIAL PRIMARY KEY,
          refutacao TEXT,
          proximidade INT,
          justificativa TEXT
      );
      ```

## Usage

1. **Database Setup**: Ensure you have a PostgreSQL database running, and provide the necessary connection details in your application (via environment variables or configuration file).

2. **Run the Application**:
    ```bash
    cargo run
    ```

3. **How It Works**:
    - The application queries the database to retrieve a list of refutations.
    - For each refutation, it sends a prompt to ChatGPT, including a text from Origen and the refutation.
    - ChatGPT responds with an SQL `INSERT` statement, which includes the proximity of the refutation to Origen's thought (on a scale of 0 to 10).
    - The application uses regular expressions to identify the SQL query structure and inserts the refutation and its proximity into the PostgreSQL database.

## Code Overview

### Modules

- **`db`**: Manages the PostgreSQL connection and queries.
- **`services::chatgpt`**: Handles communication with ChatGPT.
- **Main Function**:
  - Fetches refutations from the database.
  - Sends each refutation to ChatGPT along with Origen's text.
  - Extracts SQL queries from ChatGPT's response and stores the refutation along with the proximity score.

### Example Query

```sql
INSERT INTO public.refutacoes_origenes (refutacao, proximidade, justificativa)
VALUES('Refutação texto aqui', 7, 'Justificativa gerada aqui');
