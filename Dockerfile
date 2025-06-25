# Build stage
FROM rust:1.83-slim as builder

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar arquivos de configuração
COPY Cargo.toml ./

# Copiar código fonte
COPY src ./src

# Compilar com otimizações
ENV RUSTFLAGS="-C target-cpu=native"
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Instalar dependências runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binário compilado
COPY --from=builder /app/target/release/gow-stress-api .

# Expor porta
EXPOSE 8081

# Configurar variáveis de ambiente
ENV RUST_LOG=info
ENV PORT=8081

# Executar aplicação
CMD ["./gow-stress-api"]
