# 🎨 PixelBoard API

API para o **PixelBoard.net** — um outdoor virtual limitado onde marcas, criadores e pequenos negócios compram blocos visuais em um grande mural online de **1.000 × 1.000 pixels**.

## 🛠️ Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| Framework | Actix Web 4 |
| Linguagem | Rust 1.94 (edition 2024) |
| Banco de Dados | PostgreSQL 16 |
| Cache | Redis 7 |
| Autenticação | JWT (access + refresh tokens) |
| Documentação | Swagger UI (utoipa) |
| Containerização | Docker + Docker Compose |

## 📁 Estrutura do Projeto

```
src/
├── main.rs                      # Entry point do servidor
├── lib.rs                       # Re-exports dos módulos
├── config/                      # CORS, Swagger/OpenAPI
├── infra/                       # Pools de conexão (Postgres, Redis)
├── middlewares/                  # Auth middleware (JWT extraction)
├── modules/
│   ├── user/                    # Registro, Login, Detalhe
│   ├── block/                   # Compra, Detalhe, Edição de blocos
│   └── board/                   # Mural completo, Estatísticas
├── shared/
│   ├── exceptions/              # Error handling padronizado
│   ├── structs/                 # Query params, structs compartilhadas
│   └── treaties/                # JWT helper
└── utils/                       # Validação de body
migrations/
├── 001_create_users/
├── 002_create_blocks/
└── 003_create_reservations/
```

## 🚀 Como rodar

### Pré-requisitos

- [Docker](https://docs.docker.com/get-docker/) e [Docker Compose](https://docs.docker.com/compose/install/)

### Setup

```bash
# 1. Clone o repositório
git clone https://github.com/bush1D3v/pixel-board-api.git
cd pixel-board-api

# 2. Copie o arquivo de variáveis de ambiente
cp .env.example .env

# 3. Suba os containers
make docker-run

# 4. Acesse
# API:     http://localhost:8080
# Swagger: http://localhost:8080/docs/
```

### Desenvolvimento local (sem Docker)

```bash
# Certifique-se de ter Rust 1.94+, PostgreSQL e Redis rodando

cp .env.example .env
# Edite .env com DB_HOST=localhost, REDIS_HOST=localhost

make run
```

## 📡 Endpoints

| Método | Rota | Auth | Descrição |
|--------|------|------|-----------|
| `POST` | `/user` | ✗ | Registrar novo usuário |
| `POST` | `/user/login` | ✗ | Login (retorna JWT) |
| `GET` | `/user/{id}` | ✓ | Detalhes do usuário |
| `POST` | `/block` | ✓ | Comprar bloco no mural |
| `GET` | `/block/{id}` | ✗ | Detalhes de um bloco |
| `PATCH` | `/block/{id}` | ✓ | Editar conteúdo do bloco (dono) |
| `GET` | `/board` | ✗ | Mural completo com todos os blocos |
| `GET` | `/board/stats` | ✗ | Estatísticas do mural |
| `GET` | `/docs/` | ✗ | Swagger UI |

## 🧩 Regras de Negócio

- **Mural**: 1.000 × 1.000 pixels (1.000.000 pixels totais)
- **Bloco mínimo**: 10 × 10 pixels
- **Preço**: R$ 0,20 por pixel
- **Overlap**: blocos não podem se sobrepor
- **Edição**: apenas o dono pode alterar imagem, link, título e descrição
- **Reserva**: área reservada por 10 minutos durante o processo de compra

## 🐳 Docker

| Comando | Descrição |
|---------|-----------|
| `make docker-build` | Builda as imagens |
| `make docker-run` | Sobe os containers |
| `make docker-stop` | Para os containers |
| `make docker-clean` | Remove tudo (imagens + volumes) |
| `make docker-logs` | Logs da API em tempo real |

## 📄 Licença

[MIT](LICENSE)
