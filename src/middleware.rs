use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::auth::verificar_token;

pub async fn middleware_auth(mut requisicao: Request, proximo: Next) -> Result<Response, StatusCode> {
    let cabecalho_auth = requisicao
        .headers()
        .get(AUTHORIZATION)
        .and_then(|cabecalho| cabecalho.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !cabecalho_auth.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &cabecalho_auth[7..];
    let credenciais = verificar_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Adicionar credenciais à extensão da requisição para uso nos handlers
    requisicao.extensions_mut().insert(credenciais.id_usuario);
    requisicao.extensions_mut().insert(credenciais.email.clone());

    Ok(proximo.run(requisicao).await)
}

