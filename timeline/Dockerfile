# 1. Planejamento
FROM lukemathwalker/cargo-chef@sha256:6dce65df3d7430c797e94348b4cf36d8d5876b63ca54f35dbfd37a97c42d0add AS chef
WORKDIR /app

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2. Build das dependências
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

ENV RUSTFLAGS="-C codegen-units=1"
# ADICIONADO: CARGO_BUILD_JOBS=1 para evitar o SIGKILL (erro de memória) no Bitbucket
# Isso fará o compilador usar menos RAM, focando em um item por vez.
#RUN CARGO_BUILD_JOBS=2 cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 3. Build da aplicação
COPY . .
# Também limitamos aqui para garantir que o linkador final não estoure a RAM
#RUN CARGO_BUILD_JOBS=2 cargo build --release --package timeline
RUN cargo build --release --package timeline

# 4. Runtime: Imagem leve
FROM debian:trixie-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/timeline /app/timeline

# O App Runner usa a porta que você definir aqui, 8090 está ótimo.
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:${PORT:-8091}/ || exit 1

EXPOSE 8091

USER nobody
CMD ["./timeline"]