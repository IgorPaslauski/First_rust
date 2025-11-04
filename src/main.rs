mod auth;
mod db;
mod handlers;
mod middleware;
mod models;

use axum::{
    middleware::from_fn,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar banco de dados
    let pool = db::inicializar_banco().await?;
    println!("✅ Banco de dados inicializado");
    
    // Popular com dados iniciais
    db::popular_dados(&pool).await?;

    // Criar rotas públicas
    let rotas_publicas = Router::new()
        .route("/home", get(handlers::pagina_home))
        .route("/health", get(handlers::verificar_saude))
        .route("/api/auth/register", post(handlers::registrar))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/posts", get(handlers::obter_postagens_publicas))
        .route("/api/posts/{id}", get(handlers::obter_postagem))
        .route("/api/users", get(handlers::obter_todos_usuarios));

    // Criar rotas protegidas (requerem JWT)
    let rotas_protegidas = Router::new()
        .route("/api/profile", get(handlers::obter_perfil))
        .route("/api/posts/my", get(handlers::obter_minhas_postagens))
        .route("/api/posts", post(handlers::criar_postagem))
        .route("/api/posts/{id}", put(handlers::atualizar_postagem))
        .route("/api/posts/{id}", delete(handlers::deletar_postagem))
        .layer(from_fn(middleware::middleware_auth));

    // Aplicação principal
    let aplicacao = Router::new()
        .merge(rotas_publicas)
        .merge(rotas_protegidas)
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let endereco = "127.0.0.1:3000";
    let listener = TcpListener::bind(endereco).await?;
    
    println!("🚀 Servidor rodando em http://{endereco}");
    println!("🌐 Interface web disponível em: http://{endereco}/home");
    println!("📚 Rotas públicas:");
    println!("   GET  /home");
    println!("   GET  /health");
    println!("   POST /api/auth/register");
    println!("   POST /api/auth/login");
    println!("   GET  /api/posts");
    println!("   GET  /api/posts/{{id}}");
    println!("   GET  /api/users");
    println!("🔒 Rotas protegidas (requerem JWT):");
    println!("   GET    /api/profile");
    println!("   GET    /api/posts/my");
    println!("   POST   /api/posts");
    println!("   PUT    /api/posts/{{id}}");
    println!("   DELETE /api/posts/{{id}}");
    
    axum::serve(listener, aplicacao).await?;
    
    Ok(())
}
