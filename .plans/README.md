# Plan documents

This directory contains the working breakdown for the project plan in the repository root [PLAN.md](../PLAN.md). The plan now incorporates the answers recorded in [open-questions.md](open-questions.md).

The confirmed scope is an 8-bit H.264 encoder available through rust-av on GNU/Linux x86_64. Work has begun with the upstream submodule and workspace/build skeleton; see [PLAN.md](../PLAN.md) for progress and open implementation checks.

## Workstreams

1. [Foundation and upstream setup](01-foundation.md)
2. [Native build and FFI](02-ffi.md)
3. [Safe Rust API](03-safe-api.md)
4. [rust-av integration](04-rust-av.md)
5. [Validation and release](05-validation-release.md)

See [confirmed decisions and implementation checks](open-questions.md) for the agreed requirements and remaining repository/API inspection tasks.
