# GoldSrc.rs Standard Plugins Suite (`goldsrc-plugins-standard`)

[![CI](https://github.com/goldsrc-rs/goldsrc-plugins-standard/actions/workflows/ci.yml/badge.svg)](https://github.com/goldsrc-rs/goldsrc-plugins-standard/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

The official standard suite of high-performance WebAssembly (`wasm32-wasip1`) gameplay and administration plugins for **GoldSrc.rs** (Half-Life 1, Counter-Strike 1.6).

> **Notice:** These plugins represent the canonical production plugin implementations built on top of [`goldsrc-sdk`](https://github.com/goldsrc-rs/goldsrc-sdk). Direct live in-game tests on server environments are in active progress.

## Included Plugins

| Plugin | Bundle | Role | Description |
| :--- | :--- | :--- | :--- |
| **`administration`** | `core` | `service` | Staff registry, admin authorization (`auth:*`), ban/kick management, audit logging. |
| **`moderation`** | `gameplay` | `coordinator` | Player discipline enforcement: gag (chat/voice), mute, freeze, and temporary punishment records. |
| **`privileges`** | `gameplay` | `worker` | VIP perks, round equipment claims (AWP, grenades, armor), health regeneration, slot reservations. |
| **`chat_director`** | `gameplay` | `service` | Unified chat management, anti-flood rate limiting, staff channels (`@`), periodic broadcast rotators. |
| **`map_manager`** | `gameplay` | `coordinator` | Map voting, RTV (Rock The Vote), time limit tracking, nominate system, and map cycle management. |
| **`menu_frontend`** | `ui` | `coordinator` | Modular multi-page interactive menu hub, categories registry, debounced session dispatch. |

## Building for WebAssembly

Ensure you have Rust and the `wasm32-wasip1` target installed:

```bash
rustup target add wasm32-wasip1
```

Compile the entire plugins suite in release mode:

```bash
cargo build --workspace --target wasm32-wasip1 --release
```

Artifacts will be located in `target/wasm32-wasip1/release/*.wasm`.

## Deploying to Dedicated Server

Copy the compiled `.wasm` binaries into your server's `cstrike/addons/goldsrc/plugins/` directory and configure them in `cstrike/addons/goldsrc/plugins.toml`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines and testing standards.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
