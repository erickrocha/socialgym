# Diretrizes de Arquitetura e Padrões Rust (Módulo Workout)

Este documento serve como um guia abrangente de instruções para Agentes de IA e desenvolvedores sobre como estruturar, expandir e manter projetos em Rust utilizando o padrão estabelecido no módulo `workout`.

---

## 1. Visão Geral e Filosofia de Arquitetura

O projeto `workout` adota uma **Arquitetura em Camadas Orientada ao Domínio (Clean / Layered Architecture)** organizada em um **Cargo Workspace**.

### Princípios Fundamentais:
1. **Desacoplamento de Responsabilidades**: A lógica de negócio (`business`) não depende de framework Web (Axum) nem de protocolos de comunicação (gRPC/Tonic).
2. **Fonte da Verdade para Entidades**: As entidades da base de dados (`entity`) são desacopladas dos objetos de domínio (`domain`).
3. **Multi-Protocolo**: A camada de aplicação expõe tanto uma API REST HTTP (Axum) quanto um servidor gRPC (Tonic), ambos consumindo os mesmos Casos de Uso (`use_cases`).
4. **Persistência Sem Abstrações Excessivas**: Os `gateways` utilizam structs com métodos associados `async fn` diretos que recebem referência da conexão (`&DbConn`), evitando complexidade desnecessária com traits genéricos de repositório.

---

## 2. Estrutura do Cargo Workspace

O projeto é organizado como um workspace Cargo contendo múltiplos sub-crates:

```text
workout/
├── Cargo.toml                # Configuração do Workspace, dependências compartilhadas e profiles
├── src/
│   └── main.rs               # Entrypoint binário minimalista (invoca application::main())
├── entity/                   # Sub-crate 1: Mapeamento SeaORM de tabelas do banco de dados
├── business/                 # Sub-crate 2: Domínio, Regras de Negócio, Gateways e Mappers
├── application/              # Sub-crate 3: API HTTP REST (Axum), Rotas, Swagger (Utoipa), AppState
├── integration/              # Sub-crate 4: Servidor gRPC (Tonic), Protobufs (.proto), TLS
├── migration/                # Sub-crate 5: Gerenciador de Migrações do banco de dados (SeaORM)
```

### Configuração do `Cargo.toml` Raiz
- Define `members` do workspace e centraliza dependências em `[workspace.dependencies]`.
- Aplica otimizações de build no `[profile.release]` (`opt-level = 3`, `lto = "thin"`).

---

## 3. Detalhamento das Camadas (Sub-crates)

### 3.1. `entity` — Camada de Mapeamento ORM
- **Objetivo**: Mapeamento direto de 1-para-1 com as tabelas PostgreSQL/PostGIS usando SeaORM.
- **Regras**:
  - Cada tabela possui seu próprio arquivo em `entity/src/`.
  - Contém derives do SeaORM (`DeriveEntityModel`, `DerivePrimaryKey`, `DeriveRelation`).
  - **Não contém regras de negócio nem métodos de validação**.
  - Mantida manualmente ou atualizada para refletir as migrações de `migration/`.

### 3.2. `business` — Coração do Domínio e Regras de Negócio
Divisão interna em `business/src/`:
- **`domain/`**:
  - Structs puras em Rust que representam os conceitos de negócio (ex: `User`, `Person`, `Workout`).
  - `business_error.rs`: Define o tipo central de erro `BusinessError`, utilizado em toda a camada.
  - Enums de domínio e validações específicas de objetos.
- **`commons/`**:
  - Define o trait `EntityMapper<D, M, AM>` para padronizar a conversão entre Domínio (`D`), SeaORM Model (`M`) e ActiveModel (`AM`).
- **`gateway/`**:
  - Abstrai acessos ao banco de dados e serviços externos (S3, SQS).
  - Funções `async fn` que recebem `&DbConn` e operam com SeaORM.
  - Exemplo: `UserGateway::find_by_email(db: &DbConn, email: String) -> Result<Option<user::Model>, DbErr>`.
- **`use_cases/`**:
  - Orquestra os fluxos de aplicação.
  - Recebe conexões (`&DbConn`) e objetos de domínio.
  - Executa validações de negócio, encriptação (ex: bcrypt), chamadas aos gateways e mapeamento de retorno em `Result<T, BusinessError>`.

### 3.3. `application` — Camada de Apresentação HTTP (REST)
- **Objetivo**: Prover a API REST com Axum.
- **Estrutura interna (`application/src/`)**:
  - `http/`: Controllers HTTP que recebem payloads JSON, extraem parâmetros, chamam `UseCases` e retornam respostas HTTP.
  - `http/json/`: Structs DTO de requisição e resposta (anotadas com `serde::Deserialize`, `serde::Serialize` e `utoipa::ToSchema`).
  - `routes/`: Funções que constroem e agrupam as rotas em `Router<AppState>`.
  - `authentication/`: Middleware Axum para validação de tokens JWT.
  - `lib.rs`: Ponto de montagem onde o `AppState` (`Arc<DatabaseConnection>`) é instanciado, as migrações são executadas (`Migrator::up`), a documentação Swagger OpenAPI (`utoipa`) é gerada e o servidor `axum::serve` é iniciado.

### 3.4. `integration` — Camada de Integração gRPC
- **Objetivo**: Prover comunicação de baixa latência e tipada entre microserviços e clientes mobile via gRPC (Tonic).
- **Estrutura interna (`integration/src/`)**:
  - `proto/`: Definições `.proto` compiladas via `build.rs` (`tonic-build`).
  - `service/`: Implementações das traits geradas pelo gRPC. Mapeiam requisições Proto -> Domínio, chamam `UseCases` e convertem erros em `tonic::Status`.
  - `auth/`: Layer/Interceptor gRPC para validação de autenticação.
  - `main.rs`: Bootstrap do servidor gRPC com suporte a TLS (`rustls`/`ring`) e Tonic Reflection.

### 3.5. `migration` — Gestão de Schema do Banco
- **Objetivo**: Definir e executar migrações de banco de dados via `sea-orm-migration`.
- **Regras**: Arquivos de migração nomeados com prefixo de data (ex: `m20260129_000001_create_user_table.rs`). Executados automaticamente na inicialização da aplicação HTTP.

---

## 4. Padrões de Código e Convenções Requeridas para Agentes

### 4.1. Fluxo de Dados (Data Flow Pipeline)
Ao trafegar dados entre as camadas, o Agente DEVE respeitar a seguinte cadeia de conversão:

```text
[HTTP JSON DTO / gRPC Request]
            │
            ▼ (Conversão no Controller / Service)
      [Domain Struct] (em business/src/domain)
            │
            ▼ (Validação & Regra de Negócio em UseCase)
      [EntityMapper] (build_active_model)
            │
            ▼
   [SeaORM ActiveModel]
            │
            ▼ (Execução via Gateway no SeaORM)
     [Banco de Dados]
```

### 4.2. Tratamento de Erros
- Toda função da camada de `use_cases` DEVE retornar `Result<T, BusinessError>`.
- Erros de banco (`DbErr`) ou infraestrutura DEVEM ser tratados e convertidos em `BusinessError`:
  ```rust
  let entity = UserGateway::persist(db, user).await
      .map_err(|e| BusinessError::new(e.to_string()))?;
  ```
- Na camada HTTP (Axum), erros de negócio DEVEM ser convertidos para respostas JSON estruturadas com código HTTP adequado (ex: 400 Bad Request, 401 Unauthorized, 404 Not Found, 500 Internal Server Error).
- Na camada gRPC (Tonic), erros DEVEM ser convertidos para `tonic::Status::invalid_argument`, `tonic::Status::not_found`, etc.

### 4.3. Injeção de Dependências e Estado Shared
- O banco de dados é compartilhado via `AppState`:
  ```rust
  #[derive(Clone)]
  pub struct AppState {
      pub conn: Arc<DatabaseConnection>,
  }
  ```
- Gateways e UseCases NÃO mantêm estado interno; eles expõem métodos assíncronos que aceitam `&DbConn`.

### 4.4. Documentação de API REST com `utoipa`
- Todo controller HTTP e DTO em `application` DEVE ser documentado para OpenAPI:
  ```rust
  #[utoipa::path(
      post,
      path = "/workout/api/people",
      request_body = PersonJson,
      responses(
          (status = 200, description = "Pessoa criada com sucesso", body = PersonJson),
          (status = 400, description = "Erro de validação de dados")
      )
  )]
  pub async fn create_person(...) -> ...
  ```
- Novas rotas e schemas DEVEM ser incluídos na macro `#[derive(OpenApi)]` em `application/src/lib.rs`.

### 4.5. Estratégia de Testes Unitários com Mocks
- O projeto utiliza a feature flag `mock` para testar regras de negócio sem banco de dados real via `sea_orm::MockDatabase`.
- Ao criar novos Use Cases, o Agente DEVE fornecer testes unitários em `business/tests/mock.rs` ou sob a flag `#[cfg(feature = "mock")]`.

---

## 5. Checklist para Agente: Implementando uma Nova Funcionalidade

Ao receber uma solicitação para adicionar uma nova funcionalidade (ex: nova entidade "Treino"), o Agente DEVE seguir esta ordem rigorosa de passos:

1. **Migração (`migration/`)**: Criar o arquivo de migração criando as tabelas necessárias.
2. **Entidade (`entity/`)**: Criar a struct e enums SeaORM refletindo a tabela.
3. **Domínio & Mapper (`business/src/domain/`)**:
   - Criar a struct de Domínio pura.
   - Criar o Mapper implementando `EntityMapper<Domínio, Model, ActiveModel>`.
4. **Gateway (`business/src/gateway/`)**:
   - Implementar os métodos de consulta/persistência SeaORM (persist, update, find_by_id, etc.).
5. **Caso de Uso (`business/src/use_cases/`)**:
   - Criar a struct `XxxUseCase` com as regras de negócio e validações.
6. **Interface de Entrada**:
   - **Para REST**: Criar DTOs JSON em `application/src/http/json/`, controller em `application/src/http/`, registrar em `application/src/routes/` e adicionar anotações `utoipa` em `application/src/lib.rs`.
   - **Para gRPC**: Adicionar mensagens e serviços no arquivo `.proto` em `integration/proto/`, e implementar o serviço em `integration/src/service/`.
7. **Testes**: Escrever testes cobrindo o caso de uso e a validação de regras de negócio.
