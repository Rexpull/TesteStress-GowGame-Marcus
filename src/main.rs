use warp::Filter;
use tokio_postgres::{NoTls, Client};
use std::sync::Arc;
use std::env;

mod handlers;
mod models;
mod database;

type DbPool = Arc<Client>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@db:5432/postgres".to_string());

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Falha ao conectar ao banco de dados");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão: {}", e);
        }
    });

    let db_pool = Arc::new(client);

    database::setup_database(&db_pool).await.expect("Falha ao configurar banco de dados");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()
        .unwrap_or(8081);

    log::info!("Iniciando servidor em 0.0.0.0:{}", port);

    let db = warp::any().map(move || db_pool.clone());

    let create_programador = warp::path("programadores")
        .and(warp::post())
        .and(warp::body::json())
        .and(db.clone())
        .and_then(handlers::create_programador);

    let count_programadores = warp::path("contagem-programadores")
        .and(warp::get())
        .and(db.clone())
        .and_then(handlers::count_programadores);

    let routes = create_programador
        .or(count_programadores)
        .with(warp::log("api"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
} 