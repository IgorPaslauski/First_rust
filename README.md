# API REST com Axum, PostgreSQL e JWT

API REST completa em Rust usando Axum, PostgreSQL, JWT e bcrypt para autenticação.

## Funcionalidades

- ✅ Autenticação JWT
- ✅ Hash de senhas com bcrypt
- ✅ Múltiplas rotas REST
- ✅ Rotas públicas e protegidas
- ✅ CRUD completo de usuários e posts
- ✅ CORS habilitado
- ✅ Banco de dados PostgreSQL
- ✅ Interface web para testes (`/home`)
- ✅ Testes de carga otimizados
- ✅ Dados iniciais (seed) para testes

## Interface Web

Acesse `http://127.0.0.1:3000/home` para uma interface web completa que permite:
- Login e registro
- Visualização de posts
- Criação, edição e exclusão de posts
- Gerenciamento de perfil

## Rotas Públicas

### Home (Interface Web)
```
GET /home
```

### Health Check
```
GET /health
```

### Registro
```
POST /api/auth/register
Body:
{
  "username": "usuario",
  "email": "usuario@email.com",
  "password": "senha123"
}
```

### Login
```
POST /api/auth/login
Body:
{
  "email": "usuario@email.com",
  "password": "senha123"
}
Response:
{
  "token": "jwt_token_aqui",
  "user": { ... }
}
```

### Listar Posts Públicos
```
GET /api/posts
```

### Buscar Post por ID
```
GET /api/posts/{id}
```

### Listar Usuários
```
GET /api/users
```

## Rotas Protegidas (requerem JWT)

Adicione o header `Authorization: Bearer <token>` em todas as requisições.

### Perfil do Usuário
```
GET /api/profile
```

### Meus Posts
```
GET /api/posts/my
```

### Criar Post
```
POST /api/posts
Body:
{
  "title": "Título do Post",
  "content": "Conteúdo do post"
}
```

### Atualizar Post
```
PUT /api/posts/{id}
Body:
{
  "title": "Novo Título",
  "content": "Novo Conteúdo"
}
```

### Deletar Post
```
DELETE /api/posts/{id}
```

## Como Executar

### 1. Configurar PostgreSQL

Certifique-se de que o PostgreSQL está rodando e crie um banco de dados:

```sql
CREATE DATABASE rust;
```

**Nota:** O código está configurado para usar o banco `rust` por padrão. Você pode alterar isso em `src/db.rs` ou usando a variável de ambiente `DATABASE_URL`.

### 2. Configurar variável de ambiente (opcional)

A URL do banco está hardcoded em `src/db.rs` como:
```
postgresql://postgres:123@localhost/rust
```

Para usar uma URL diferente, configure a variável `DATABASE_URL` ou modifique o código:

```bash
# Windows PowerShell
$env:DATABASE_URL="postgresql://usuario:senha@localhost/nome_banco"

# Linux/Mac
export DATABASE_URL="postgresql://usuario:senha@localhost/nome_banco"
```

### 3. Executar a aplicação

```bash
# Modo desenvolvimento
cargo run

# Modo release (otimizado)
cargo run --release
```

O servidor irá rodar em `http://127.0.0.1:3000`

**Nota:** 
- As tabelas serão criadas automaticamente na primeira execução
- Dados iniciais (3 usuários e 6 posts) serão inseridos automaticamente se o banco estiver vazio
- Credenciais de teste:
  - `admin@example.com` / `admin123`
  - `joao@example.com` / `senha123`
  - `maria@example.com` / `password`

## Exemplo de Uso

### 1. Registrar usuário
```bash
curl -X POST http://127.0.0.1:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"joao","email":"joao@email.com","password":"123456"}'
```

### 2. Fazer login
```bash
curl -X POST http://127.0.0.1:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"joao@email.com","password":"123456"}'
```

### 3. Criar post (com token)
```bash
curl -X POST http://127.0.0.1:3000/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer SEU_TOKEN_AQUI" \
  -d '{"title":"Meu Post","content":"Conteúdo do post"}'
```

## Testes de Carga

O projeto inclui dois exemplos de teste de carga:

### 1. Teste de Carga Público (`teste_carga.rs`)

Testa o endpoint público `/api/posts`:

```bash
# Padrão: 200.000 requisições, 1.500 concorrência
cargo run --example teste_carga --release

# Customizado
$env:N="50000"; $env:C="500"; cargo run --example teste_carga --release
```

**Variáveis de ambiente:**
- `N`: Total de requisições (padrão: 200000)
- `C`: Concorrência (padrão: 1500)
- `URL`: URL do endpoint (padrão: `http://127.0.0.1:3000/api/posts`)
- `WARMUP`: Requisições de warmup (padrão: 20000)

### 2. Teste de Carga com Autenticação (`teste_carga_auth.rs`)

Testa o endpoint protegido `/api/posts/my` com JWT:

```bash
# Padrão: 20.000 requisições, 200 concorrência
cargo run --example teste_carga_auth --release

# Customizado
$env:N="50000"; $env:C="500"; cargo run --example teste_carga_auth --release

# Com token personalizado
$env:TOKEN="seu_token_jwt_aqui"; cargo run --example teste_carga_auth --release
```

**Variáveis de ambiente:**
- `N`: Total de requisições (padrão: 20000)
- `C`: Concorrência (padrão: 200)
- `URL`: URL do endpoint (padrão: `http://127.0.0.1:3000/api/posts/my`)
- `TOKEN`: Token JWT (padrão: token de teste pré-configurado)
- `WARMUP`: Requisições de warmup (padrão: 2000)

**Importante:** Certifique-se de que o servidor está rodando antes de executar os testes!

## Variáveis de Ambiente

**Obrigatório:**
- `DATABASE_URL`: URL do banco PostgreSQL (pode ser configurada em `src/db.rs` ou via variável de ambiente)

**Opcional:**
- `JWT_SECRET`: Chave secreta para JWT (padrão: `your-secret-key-change-in-production`)

## Estrutura do Projeto

```
hello_rust/
├── src/
│   ├── main.rs          # Ponto de entrada, configuração de rotas
│   ├── models.rs        # Estruturas de dados (Usuario, Postagem)
│   ├── db.rs            # Inicialização do banco e seed
│   ├── auth.rs          # JWT e hash de senhas
│   ├── middleware.rs    # Middleware de autenticação
│   └── handlers.rs      # Handlers das rotas REST
├── static/
│   └── home.html        # Interface web frontend
├── examples/
│   ├── teste_carga.rs       # Teste de carga para endpoint público
│   └── teste_carga_auth.rs  # Teste de carga para endpoint protegido
└── Cargo.toml           # Dependências do projeto
```

## Tecnologias

- **Axum 0.8**: Framework web assíncrono
- **Tokio**: Runtime assíncrono
- **SQLx**: Query builder type-safe
- **PostgreSQL**: Banco de dados relacional
- **JWT**: Autenticação com JSON Web Tokens
- **Bcrypt**: Hash de senhas
- **Chrono**: Manipulação de datas
- **Serde**: Serialização/deserialização
- **Reqwest**: Cliente HTTP para testes de carga
- **Futures**: Streams assíncronos para concorrência

## Dados Iniciais

Ao iniciar pela primeira vez (ou quando o banco estiver vazio), o sistema automaticamente cria:

**3 Usuários:**
- `admin` (admin@example.com / admin123)
- `joao` (joao@example.com / senha123)
- `maria` (maria@example.com / password)

**6 Posts:**
- 2 posts do admin
- 2 posts do joao
- 2 posts da maria

Esses dados podem ser usados para testes imediatos sem necessidade de registro manual.

