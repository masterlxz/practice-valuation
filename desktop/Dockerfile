FROM node:20-bookworm-slim

# Bibliotecas do Linux que o Tauri precisa:
# - libwebkit2gtk: motor HTML que renderiza o React dentro da janela
# - libgtk-3-dev: toolkit de janelas do Linux
# - libssl-dev: criptografia (HTTPS)
# - build-essential: compilador C (dependências do Rust precisam)
# - as demais: ícones, indicadores de sistema, imagens SVG
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    file \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf \
    && rm -rf /var/lib/apt/lists/*

# Instala Rust dentro do container
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# sea-orm-cli: ferramenta de dev pra gerar migrations/entities do banco (SeaORM).
# Não é dependência do projeto (isso já está no Cargo.toml) — fica só no ambiente,
# igual o próprio cargo/rustc.
RUN cargo install sea-orm-cli

# Python + venv, pra rodar o coletor de dados (../data-collector) chamado pelo app
RUN apt-get update && apt-get install -y \
    python3 \
    python3-venv \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
