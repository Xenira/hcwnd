FROM rust:alpine AS chef
USER root
RUN <<EOL
set -e
apk update
apk add --no-cache openssl-dev musl-dev libpq-dev
cargo install cargo-chef --locked
EOL
WORKDIR /app

FROM rust AS dx-chef
USER root
RUN cargo install cargo-chef --locked
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin server

FROM dx-chef AS dx-planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS server-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release
RUN cargo build --release --bin server

FROM dx-chef AS web-builder
COPY --from=dx-planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --no-build
COPY . .
RUN cargo binstall dioxus-cli --no-confirm
RUN dx bundle --web --package web --fullstack false --release --debug-symbols false

FROM alpine AS combiner
WORKDIR /app
COPY --from=server-builder /app/target/release/server ./server
COPY --from=web-builder /app/target/dx/web/release/web/public ./public

FROM alpine
WORKDIR /app
USER 1000
COPY --from=combiner /app .
