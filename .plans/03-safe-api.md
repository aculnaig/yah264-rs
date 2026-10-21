# 03 — Safe `yah264` API

## Outcomes

- A safe, idiomatic Rust interface for configuring and using the encoder.
- Explicit frame, output, error, and resource ownership behavior.
- A documented memory model, including justified `zerocopy` use.

## Tasks

1. Start with 8-bit YUV420P and the simplest upstream-supported H.264 settings. Accept three planes with per-plane strides, require even dimensions initially, and pass through color metadata only when both APIs support it.
2. Expose dimensions and any settings required by rust-av/upstream; use upstream defaults for the rest.
3. Define frame input ownership and whether the native encoder consumes data synchronously or retains it; accept caller-prepared frames without conversion.
4. Expose output NAL payloads as borrowed slices tied to the encoder, because yah264 invalidates them on the next API call. The rust-av adapter copies into its required `Vec<u8>` packet storage.
5. Wrap native resources with deterministic cleanup and typed errors that retain native status codes.
6. Require exclusive mutable access; implement `Send` only if supported by upstream and avoid `Sync` without explicit guarantees.
7. Use `zerocopy` where representation and alignment are verified, and satisfy the zero-copy requirement wherever lifetimes and ownership permit; document unavoidable copy paths and their costs.
8. Deny `unsafe_op_in_unsafe_fn`, document unsafe blocks, and require `# Safety` sections on public unsafe functions.
9. Add API docs, examples, and unit tests for valid/invalid inputs and failure cleanup.

## Acceptance

- Safe callers do not need `unsafe` for supported encoder use.
- Invalid configurations and buffers fail before unsafe native access where practical.
- Lifetimes and thread guarantees match the native implementation.
- Input planes are borrowed without copying; output copies at the rust-av packet boundary are documented.
