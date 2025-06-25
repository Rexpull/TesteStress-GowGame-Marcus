use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct CreateProgramadorRequest {
    pub apelido: String,
    pub nome: String,
    pub nascimento: String,
    pub stack: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct Programador {
    pub id: Uuid,
    pub apelido: String,
    pub nome: String,
    pub nascimento: NaiveDate,
    pub stack: Option<Vec<String>>,
}

impl CreateProgramadorRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.apelido.is_empty() || self.apelido.len() > 32 {
            return Err("Apelido deve ter entre 1 e 32 caracteres".to_string());
        }

        if self.nome.is_empty() || self.nome.len() > 100 {
            return Err("Nome deve ter entre 1 e 100 caracteres".to_string());
        }

        if self.nascimento.is_empty() {
            return Err("Data de nascimento é obrigatória".to_string());
        }

        NaiveDate::parse_from_str(&self.nascimento, "%Y-%m-%d")
            .map_err(|_| "Data de nascimento deve estar no formato AAAA-MM-DD".to_string())?;

        if let Some(stack) = &self.stack {
            for item in stack {
                if item.len() > 32 {
                    return Err("Cada item da stack deve ter no máximo 32 caracteres".to_string());
                }
            }
        }

        Ok(())
    }

    pub fn to_programador(&self) -> Result<Programador, String> {
        self.validate()?;
        
        let nascimento = NaiveDate::parse_from_str(&self.nascimento, "%Y-%m-%d")
            .map_err(|_| "Formato de data inválido".to_string())?;

        Ok(Programador {
            id: Uuid::new_v4(),
            apelido: self.apelido.clone(),
            nome: self.nome.clone(),
            nascimento,
            stack: self.stack.clone(),
        })
    }
} 