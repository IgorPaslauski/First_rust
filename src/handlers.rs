use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    response::{Json, Html},
};
use crate::{
    auth::{criar_token, hash_senha, verificar_senha},
    db::DbPool,
    models::*,
};

// ========== Rotas Públicas ==========

pub async fn pagina_home() -> Html<&'static str> {
    Html(include_str!("../static/home.html"))
}

pub async fn verificar_saude() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "API está funcionando"
    }))
}

pub async fn registrar(
    State(pool): State<DbPool>,
    Json(requisicao): Json<CriarUsuarioRequisicao>,
) -> Result<Json<RespostaUsuario>, StatusCode> {
    // Verificar se email já existe
    let existente = sqlx::query_as::<_, Usuario>(
        "SELECT * FROM users WHERE email = $1 OR username = $1"
    )
    .bind(&requisicao.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existente.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let hash_senha = hash_senha(&requisicao.senha)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let usuario = sqlx::query_as::<_, Usuario>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&requisicao.nome_usuario)
    .bind(&requisicao.email)
    .bind(&hash_senha)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RespostaUsuario::from(usuario)))
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(requisicao): Json<LoginRequisicao>,
) -> Result<Json<RespostaLogin>, StatusCode> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&requisicao.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    if !verificar_senha(&requisicao.senha, &usuario.hash_senha)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = criar_token(usuario.id, usuario.email.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RespostaLogin {
        token,
        usuario: RespostaUsuario::from(usuario),
    }))
}

pub async fn obter_postagens_publicas(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<RespostaPostagem>>, StatusCode> {
    let postagens = sqlx::query_as::<_, Postagem>(
        "SELECT * FROM posts ORDER BY created_at DESC LIMIT 10"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let resposta_postagens: Vec<RespostaPostagem> = postagens.into_iter().map(RespostaPostagem::from).collect();
    Ok(Json(resposta_postagens))
}

// ========== Rotas Protegidas ==========

pub async fn obter_perfil(
    Extension(id_usuario): Extension<i32>,
    State(pool): State<DbPool>,
) -> Result<Json<RespostaUsuario>, StatusCode> {
    let usuario = sqlx::query_as::<_, Usuario>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id_usuario)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(RespostaUsuario::from(usuario)))
}

pub async fn obter_minhas_postagens(
    Extension(id_usuario): Extension<i32>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<RespostaPostagem>>, StatusCode> {
    let postagens = sqlx::query_as::<_, Postagem>(
        "SELECT * FROM posts WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(id_usuario)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let resposta_postagens: Vec<RespostaPostagem> = postagens.into_iter().map(RespostaPostagem::from).collect();
    Ok(Json(resposta_postagens))
}

pub async fn criar_postagem(
    Extension(id_usuario): Extension<i32>,
    State(pool): State<DbPool>,
    Json(requisicao): Json<CriarPostagemRequisicao>,
) -> Result<Json<RespostaPostagem>, StatusCode> {
    let postagem = sqlx::query_as::<_, Postagem>(
        r#"
        INSERT INTO posts (title, content, user_id)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&requisicao.titulo)
    .bind(&requisicao.conteudo)
    .bind(id_usuario)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RespostaPostagem::from(postagem)))
}

pub async fn obter_postagem(
    Path(id_postagem): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<Json<RespostaPostagem>, StatusCode> {
    let postagem = sqlx::query_as::<_, Postagem>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(id_postagem)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(RespostaPostagem::from(postagem)))
}

pub async fn atualizar_postagem(
    Extension(id_usuario): Extension<i32>,
    Path(id_postagem): Path<i32>,
    State(pool): State<DbPool>,
    Json(requisicao): Json<CriarPostagemRequisicao>,
) -> Result<Json<RespostaPostagem>, StatusCode> {
    // Verificar se o post pertence ao usuário e atualizar
    let postagem_atualizada = sqlx::query_as::<_, Postagem>(
        r#"
        UPDATE posts
        SET title = $1, content = $2
        WHERE id = $3 AND user_id = $4
        RETURNING *
        "#,
    )
    .bind(&requisicao.titulo)
    .bind(&requisicao.conteudo)
    .bind(id_postagem)
    .bind(id_usuario)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(RespostaPostagem::from(postagem_atualizada)))
}

pub async fn deletar_postagem(
    Extension(id_usuario): Extension<i32>,
    Path(id_postagem): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<StatusCode, StatusCode> {
    let resultado = sqlx::query(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2"
    )
    .bind(id_postagem)
    .bind(id_usuario)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if resultado.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn obter_todos_usuarios(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<RespostaUsuario>>, StatusCode> {
    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let resposta_usuarios: Vec<RespostaUsuario> = usuarios.into_iter().map(RespostaUsuario::from).collect();
    Ok(Json(resposta_usuarios))
}
