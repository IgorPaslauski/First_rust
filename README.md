# API REST com Axum, SQLite e JWT

API REST completa em Rust usando Axum, SQLite, JWT e bcrypt para autenticação.

## Funcionalidades

- ✅ Autenticação JWT
- ✅ Hash de senhas com bcrypt
- ✅ Múltiplas rotas REST
- ✅ Rotas públicas e protegidas
- ✅ CRUD completo de usuários e posts
- ✅ CORS habilitado
- ✅ Banco de dados PostgreSQL

## Rotas Públicas

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
GET /api/posts/:id
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
PUT /api/posts/:id
Body:
{
  "title": "Novo Título",
  "content": "Novo Conteúdo"
}
```

### Deletar Post
```
DELETE /api/posts/:id
```

## Como Executar

### 1. Configurar PostgreSQL

Certifique-se de que o PostgreSQL está rodando e crie um banco de dados:

```sql
CREATE DATABASE hello_rust;
```

### 2. Configurar variável de ambiente

Configure a variável `DATABASE_URL`:

```bash
# Windows PowerShell
$env:DATABASE_URL="postgresql://usuario:senha@localhost/hello_rust"

# Linux/Mac
export DATABASE_URL="postgresql://usuario:senha@localhost/hello_rust"
```

### 3. Executar a aplicação

```bash
cargo run
```

O servidor irá rodar em `http://127.0.0.1:3000`

**Nota:** As tabelas serão criadas automaticamente na primeira execução.

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

## Variáveis de Ambiente

**Obrigatório:**
- `DATABASE_URL`: URL do banco PostgreSQL (ex: `postgresql://usuario:senha@localhost/hello_rust`)

**Opcional:**
- `JWT_SECRET`: Chave secreta para JWT (padrão: `your-secret-key-change-in-production`)

## Tecnologias

- **Axum**: Framework web assíncrono
- **SQLx**: Query builder type-safe
- **PostgreSQL**: Banco de dados relacional
- **JWT**: Autenticação com JSON Web Tokens
- **Bcrypt**: Hash de senhas
- **Chrono**: Manipulação de datas
- **Serde**: Serialização/deserialização

