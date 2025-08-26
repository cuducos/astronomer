# Contributing

Get a [personal access token from GitHub](https://github.com/settings/tokens) with `public_repo` and `read_user` scopes, and save it as an environment variable called `ASTRONOMER_GITHUB_TOKEN`.

The Rust codebase has frontend and backend code.

## Compiling the frontend WASM

Maybe you'll need `cargo install wasm-bindgen-cli`:

```console
$ cargo build --target wasm32-unknown-unknown --workspace --exclude backend --exclude shared --release
$ wasm-bindgen --target web --out-dir static/ target/wasm32-unknown-unknown/release/frontend.wasm 
```

## Running the backend

```console
$ cargo run --bin backend
```
