# yah264-rs

Rust bindings and an adapter for the [yah264](https://github.com/terranvigil/yah264) H.264 encoder and [rust-av](https://github.com/rust-av/rust-av).

## Crates

- `yah264-sys`: generated FFI bindings and the native static build.
- `yah264`: safe Rust encoder API.
- `av-yah264`: rust-av `Encoder` adapter.

The initial target is GNU/Linux x86_64 with 8-bit YUV420P input. The initial native build compiles upstream's scalar C source set into a static archive; SIMD tiers are not included yet.

## Checkout and build prerequisites

Clone with submodules initialized, or run `git submodule update --init --recursive`. Building requires a C11-capable GCC, clang/libclang for bindgen, and the Rust toolchain required by the workspace dependencies. Cargo dependencies are declared in the manifests; build scripts do not fetch source or binaries.

The yah264 submodule tracks upstream `main` and has its own `GPL-2.0-or-later` license. The Rust code in this repository is intended to be dual-licensed under MIT or Apache-2.0. Review the upstream license requirements before distributing binaries that include the statically linked encoder.

## Status

Workspace and native FFI setup are in progress. See [PLAN.md](PLAN.md) for implementation stages and current assumptions.
