# Pocket-ic Live issue

## Description

This repository shows the code for the Pocket-ic Live issue.

The issue is that the canister is doing a HTTP outcall to a local service located at `echo-http/` which just echoes back the request body.

The issue is that when running with pocket-ic, the echo-http server receives the request and sends the response back, the canister receives the response, but pocket-ic just crashes with the following error:

```txt
thread 'test_should_get_posts_1' panicked at /home/veeso/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pocket-ic-6.0.0/src/nonblocking.rs:1304:51:
BadIngressMessage("Failed to answer to ingress 0x35a3b8c5fde5ef5305fa502edf15d9bd5cafc84f9c6a5148a28d6a00c51aed5a after 100 rounds.")
```

## Requirements

- Rust >= 1.83
- Just <https://github.com/casey/just>
- wasm target:

    ```sh
    rustup target add wasm32-unknown-unknown
    ```

- Node.js >= 18
- candid-extractor

    ```sh
    cargo install candid-extractor
    ```

- ic-wasm

    ```sh
    cargo install ic-wasm
    ```

## Steps to reproduce

### Run the echo-http server

```sh
cd echo-http
yarn || npm install
node index.js
```

### Run tests

```sh
just build
just test
```
