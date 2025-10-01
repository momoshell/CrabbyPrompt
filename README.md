# CrabbyPrompt
Trying to build a localy deployed LLM source language models, on a full Rust web stack

# Leptos SSR
Since this project uses Leptos for server-side rendering, we need to grab build tools.

```bash
cargo install --locked cargo-leptos
```

## Running project in DEV mode

```bash
cargo leptos watch
```

## Running project in PROD mode
Run the build command to get the server binary in target/release and site package in target/site and to start the server:
```bash
cargo leptos serve --release
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)
5. Run `npm install` in end2end subdirectory before test
