# syntax=docker/dockerfile:1.7-labs
FROM mcr.microsoft.com/devcontainers/rust:latest

# キャッシュ効率を上げるためレイヤーを分割
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

# 必要なパッケージを一括インストール
RUN apt-get -y install --no-install-recommends \
    postgresql-client \
	build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Cargoキャッシュディレクトリを事前作成
RUN mkdir -p /usr/local/cargo/registry \
    && mkdir -p /usr/local/cargo/git \
    && chown -R vscode:vscode /usr/local/cargo

# ターゲットディレクトリを作成して権限を設定
RUN mkdir -p /workspaces/target \
    && chown -R vscode:vscode /workspaces

# Cargoツールをインストール
RUN --mount=type=cache,target=/usr/local/cargo/registry,uid=1000,gid=1000 \
    --mount=type=cache,target=/usr/local/cargo/git,uid=1000,gid=1000 \
    cargo install sea-orm-cli cargo-insta loco cargo-watch

# 環境変数ファイルをコピー
COPY .env /.env

# キャッシュディレクトリの権限設定
RUN chown -R vscode:vscode /usr/local/cargo

# 作業ディレクトリを設定
WORKDIR /workspaces