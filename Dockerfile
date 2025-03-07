FROM rust:1-slim-bullseye AS build
WORKDIR /astronomer
COPY Cargo.toml .
COPY Cargo.lock .
COPY backend/ backend
COPY frontend/ frontend
RUN mkdir static && \
    cargo install wasm-bindgen-cli && \
    apt-get -y update && apt-get install -y libssl-dev pkg-config && \
    cargo build --workspace --exclude frontend --release && \
    cp target/release/backend /usr/local/bin/backend && \
    rustup target add wasm32-unknown-unknown && \
    cargo build --target wasm32-unknown-unknown --workspace --exclude backend --release && \
    wasm-bindgen --target web --out-dir ./static/ target/wasm32-unknown-unknown/release/frontend.wasm && \
    cargo clean && \
    rm -rf /var/lib/apt/lists/*

FROM debian:bullseye-slim
WORKDIR /astronomer
COPY static/app.js static/app.js
COPY --from=build /astronomer/static/frontend* static/
COPY --from=build /usr/local/bin/backend /usr/local/bin/backend
EXPOSE 8080
CMD ["backend"]
