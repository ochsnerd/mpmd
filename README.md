# mpmd

A [Dioxus](https://dioxuslabs.com) 0.7 fullstack web app (Rust + WebAssembly).

## Dev environment (Nix flake)

`flake.nix` provides a dev shell with:

- Rust stable (via `rust-overlay`), including the `wasm32-unknown-unknown` target, `rust-src` and `rust-analyzer`
- `dx` (dioxus-cli)
- `wasm-bindgen-cli`, `binaryen`, `pkg-config`, `openssl`

Versions are pinned by `flake.lock` (currently Rust 1.99.1, dx 0.7.10).

With direnv, the shell loads automatically when you `cd` into the project (`.envrc` contains `use flake`; run `direnv allow` once). Without direnv:

```sh
nix develop
```

Update the pinned toolchain with `nix flake update`.

## Dioxus commands

```sh
dx serve                      # dev server with hot reload (http://localhost:8080)
dx build --platform web       # debug build into target/dx/
dx bundle --web --release     # optimized release bundle (server + public/)
dx fmt                        # format rsx! blocks
dx check                      # lint rsx! blocks
```

The server bundle ends up in `target/dx/mpmd/release/web/`.

## Docker

The `Dockerfile` follows the [official Dioxus deploy guide](https://dioxuslabs.com/learn/0.7/tutorial/deploy). It pins Rust 1.99.1 and dx 0.7.10 to match the flake.

```sh
docker build -t mpmd .
docker run --rm -p 8080:8080 mpmd
```

Then open http://localhost:8080. The server reads `IP` and `PORT` from the environment (defaults in the image: `0.0.0.0`, `8080`).
