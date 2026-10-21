# 02 — Native build and `yah264-sys`

## Outcomes

- A reproducible native build of upstream yah264 from `yah264-sys/build.rs`.
- Rust declarations that match the selected native ABI.
- A narrow, documented unsafe interface for use by the safe wrapper.

## Tasks

1. Identify upstream compilation units, required native dependencies, compiler flags, generated files, and platform conditionals.
2. Build through `cc`, as requested; identify the minimum GCC version needed by upstream.
3. Generate bindings with bindgen and document clang/libclang prerequisites.
4. Implement target-aware build logic and emit actionable errors for missing tools or unsupported targets.
5. Define FFI ownership and lifetime contracts; document every unsafe operation and native return/status code.
6. Add unit tests for wrapper-side invariants; leave dedicated FFI/integration test coverage as a later extension.

## Acceptance

- GNU/Linux x86_64 compiles the native library and sys crate reproducibly with the required GCC.
- Bindings correspond to the checked-out upstream revision.
- Native memory and handle ownership are explicit and reviewable.
