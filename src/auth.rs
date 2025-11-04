use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &str = "your-secret-key-change-in-production"; // Em produção, use variável de ambiente

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credenciais {
    pub id_usuario: i32,
    pub email: String,
    pub exp: usize,
}

pub fn criar_token(id_usuario: i32, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiracao = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600 * 24; // 24 horas

    let credenciais = Credenciais {
        id_usuario,
        email,
        exp: expiracao,
    };

    let secreto = std::env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET.to_string());
    encode(
        &Header::default(),
        &credenciais,
        &EncodingKey::from_secret(secreto.as_ref()),
    )
}

pub fn verificar_token(token: &str) -> Result<Credenciais, jsonwebtoken::errors::Error> {
    let secreto = std::env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET.to_string());
    let dados_token = decode::<Credenciais>(
        token,
        &DecodingKey::from_secret(secreto.as_ref()),
        &Validation::default(),
    )?;
    Ok(dados_token.claims)
}

pub fn hash_senha(senha: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(senha, bcrypt::DEFAULT_COST)
}

pub fn verificar_senha(senha: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(senha, hash)
}

