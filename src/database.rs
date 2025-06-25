use tokio_postgres::Client;
use crate::models::Programador;
use anyhow::Result;

pub async fn setup_database(client: &Client) -> Result<()> {
    client
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS programadores (
                id UUID PRIMARY KEY,
                apelido VARCHAR(32) UNIQUE NOT NULL,
                nome VARCHAR(100) NOT NULL,
                nascimento DATE NOT NULL,
                stack TEXT[]
            )
            "#,
            &[],
        )
        .await?;

    client
        .execute(
            r#"
            CREATE INDEX IF NOT EXISTS idx_programadores_stack 
            ON programadores USING GIN (stack)
            "#,
            &[],
        )
        .await?;

    client
        .execute(
            r#"
            CREATE INDEX IF NOT EXISTS idx_programadores_apelido 
            ON programadores (apelido)
            "#,
            &[],
        )
        .await?;

    log::info!("Banco de dados configurado com sucesso");
    Ok(())
}

pub async fn insert_programador(client: &Client, programador: &Programador) -> Result<()> {
    let stack_values: Option<Vec<&str>> = programador.stack.as_ref().map(|s| s.iter().map(|x| x.as_str()).collect());
    
    client
        .execute(
            r#"
            INSERT INTO programadores (id, apelido, nome, nascimento, stack)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            &[
                &programador.id,
                &programador.apelido,
                &programador.nome,
                &programador.nascimento,
                &stack_values,
            ],
        )
        .await?;

    Ok(())
}

pub async fn count_programadores(client: &Client) -> Result<i64> {
    let row = client
        .query_one("SELECT COUNT(*) as count FROM programadores", &[])
        .await?;
    
    let count: i64 = row.get("count");
    Ok(count)
} 