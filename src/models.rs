use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    #[sqlx(rename = "username")]
    pub nome_usuario: String,
    pub email: String,
    #[sqlx(rename = "password_hash")]
    pub hash_senha: String,
    #[sqlx(rename = "created_at")]
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespostaUsuario {
    pub id: i32,
    pub nome_usuario: String,
    pub email: String,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl From<Usuario> for RespostaUsuario {
    fn from(usuario: Usuario) -> Self {
        RespostaUsuario {
            id: usuario.id,
            nome_usuario: usuario.nome_usuario,
            email: usuario.email,
            criado_em: usuario.criado_em,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CriarUsuarioRequisicao {
    pub nome_usuario: String,
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequisicao {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Serialize)]
pub struct RespostaLogin {
    pub token: String,
    pub usuario: RespostaUsuario,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Postagem {
    pub id: i32,
    #[sqlx(rename = "title")]
    pub titulo: String,
    #[sqlx(rename = "content")]
    pub conteudo: String,
    #[sqlx(rename = "user_id")]
    pub id_usuario: i32,
    #[sqlx(rename = "created_at")]
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CriarPostagemRequisicao {
    pub titulo: String,
    pub conteudo: String,
}

#[derive(Debug, Serialize)]
pub struct RespostaPostagem {
    pub id: i32,
    pub titulo: String,
    pub conteudo: String,
    pub id_usuario: i32,
    pub criado_em: chrono::DateTime<chrono::Utc>,
}

impl From<Postagem> for RespostaPostagem {
    fn from(postagem: Postagem) -> Self {
        RespostaPostagem {
            id: postagem.id,
            titulo: postagem.titulo,
            conteudo: postagem.conteudo,
            id_usuario: postagem.id_usuario,
            criado_em: postagem.criado_em,
        }
    }
}

