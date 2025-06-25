use warp::{Reply, Rejection, http::StatusCode};
use crate::models::CreateProgramadorRequest;
use crate::database;
use crate::DbPool;

pub async fn create_programador(
    programador: CreateProgramadorRequest,
    pool: DbPool,
) -> Result<impl Reply, Rejection> {
    let programador_data = match programador.to_programador() {
        Ok(data) => data,
        Err(err) => {
            return Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({
                    "error": err
                })),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    match database::insert_programador(&pool, &programador_data).await {
        Ok(_) => {
            let location_header = format!("/programadores/{}", programador_data.id);
            let response = warp::reply::with_header(
                warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({
                        "id": programador_data.id,
                        "apelido": programador_data.apelido,
                        "nome": programador_data.nome,
                        "nascimento": programador_data.nascimento.format("%Y-%m-%d").to_string(),
                        "stack": programador_data.stack
                    })),
                    StatusCode::CREATED,
                ),
                "Location",
                location_header,
            );
            
            Ok(response)
        }
        Err(err) => {
            let error_msg = err.to_string();
            if error_msg.contains("duplicate key") || error_msg.contains("unique") {
                Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({
                        "error": "Apelido já existe"
                    })),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ))
            } else {
                log::error!("Erro no banco de dados: {:?}", err);
                Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({
                        "error": "Erro interno do servidor"
                    })),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}

pub async fn count_programadores(pool: DbPool) -> Result<impl Reply, Rejection> {
    match database::count_programadores(&pool).await {
        Ok(count) => {
            Ok(warp::reply::with_status(
                count.to_string(),
                StatusCode::OK,
            ))
        }
        Err(err) => {
            log::error!("Erro ao contar programadores: {:?}", err);
            Ok(warp::reply::with_status(
                "Erro interno do servidor".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
} 