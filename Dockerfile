FROM rust:1-slim-bullseye AS frontend
WORKDIR /astronomer
COPY Cargo.toml .
COPY Cargo.lock .
COPY backend/ backend
COPY frontend/ frontend
RUN mkdir static && \
    cargo install wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown && \
    cargo build --target wasm32-unknown-unknown --workspace --exclude backend --release && \
    wasm-bindgen --target web --out-dir ./static/ target/wasm32-unknown-unknown/release/frontend.wasm && \
    cargo clean

FROM rust:1-slim-bullseye AS backend
WORKDIR /astronomer
COPY Cargo.toml .
COPY Cargo.lock .
COPY backend/ backend
COPY frontend/ frontend
RUN apt-get -y update && apt-get install -y libssl-dev pkg-config && \
    cargo build --workspace --exclude frontend --release && \
    cp target/release/backend /usr/local/bin/ && \
    cargo clean && \
    rm -rf /var/lib/apt/lists/*


FROM debian:bullseye-slim
WORKDIR /astronomer
RUN apt-get -y update && apt-get install -y ca-certificates
COPY static/app.js static/app.js
COPY --from=frontend /astronomer/static/frontend* static/
COPY --from=backend /usr/local/bin/backend /usr/local/bin/backend
CMD ["backend"]
