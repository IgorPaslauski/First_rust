use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::auth::hash_senha;

pub type DbPool = Pool<Postgres>;

pub async fn inicializar_banco() -> Result<DbPool, sqlx::Error> {
    // PostgreSQL connection string: postgresql://user:password@localhost/dbname
    let url_banco = "postgresql://postgres:123@localhost/rust";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_banco)
        .await?;

    // Criar tabelas
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn popular_dados(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Verificar se já existem usuários
    let contagem_usuarios: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if contagem_usuarios > 0 {
        println!("📊 Banco de dados já possui dados. Pulando seed.");
        return Ok(());
    }

    println!("🌱 Populando banco de dados com dados iniciais...");

    // Criar usuários de teste
    let usuarios = vec![
        ("admin", "admin@example.com", "admin123"),
        ("joao", "joao@example.com", "senha123"),
        ("maria", "maria@example.com", "password"),
    ];

    let mut ids_usuarios = Vec::new();

    for (nome_usuario, email, senha) in usuarios {
        let hash_senha = hash_senha(senha)
            .map_err(|e| sqlx::Error::Decode(format!("Erro ao fazer hash da senha: {}", e).into()))?;

        let id_usuario: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(nome_usuario)
        .bind(email)
        .bind(&hash_senha)
        .fetch_one(pool)
        .await?;

        ids_usuarios.push((id_usuario, nome_usuario));
        println!("   ✅ Usuário criado: {} (id: {})", nome_usuario, id_usuario);
    }

    // Criar postagens de teste
    let postagens = vec![
        (ids_usuarios[0].0, "Bem-vindo ao Blog!", "Este é o primeiro post do nosso blog. Aqui você pode compartilhar suas ideias e experiências."),
        (ids_usuarios[0].0, "Dicas de Rust", "Rust é uma linguagem de programação incrível! Algumas dicas: use ownership, aproveite os borrows, e não tenha medo do compilador."),
        (ids_usuarios[1].0, "Meu primeiro post", "Olá! Sou o João e este é meu primeiro post na plataforma. Estou muito animado!"),
        (ids_usuarios[1].0, "Trabalhando com Axum", "Axum é um framework web moderno para Rust. É incrivelmente rápido e type-safe!"),
        (ids_usuarios[2].0, "Hello World!", "Olá mundo! Este é um post de teste da Maria."),
        (ids_usuarios[2].0, "PostgreSQL é fantástico", "PostgreSQL é um dos melhores bancos de dados relacionais disponíveis. É open-source e muito poderoso!"),
    ];

    let contagem_postagens = postagens.len();

    for (id_usuario, titulo, conteudo) in &postagens {
        let id_postagem: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO posts (title, content, user_id)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(*titulo)
        .bind(*conteudo)
        .bind(*id_usuario)
        .fetch_one(pool)
        .await?;

        let nome_usuario = ids_usuarios.iter().find(|(id, _)| *id == *id_usuario).map(|(_, u)| *u).unwrap_or("unknown");
        println!("   ✅ Post criado: \"{}\" por {} (id: {})", titulo, nome_usuario, id_postagem);
    }

    println!("✨ Seed concluído! {} usuários e {} posts criados.", ids_usuarios.len(), contagem_postagens);
    println!("\n📝 Credenciais de teste:");
    println!("   👤 admin@example.com / admin123");
    println!("   👤 joao@example.com / senha123");
    println!("   👤 maria@example.com / password");

    Ok(())
}

