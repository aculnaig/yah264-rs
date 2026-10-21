# 05 — Validation and release

## Outcomes

- Repeatable validation on the agreed Rust/native target matrix.
- User-facing documentation, examples, and release metadata.

## Tasks

1. Start with unit tests, leaving room for FFI and encode/decode coverage later.
2. Use GitHub Actions on Ubuntu x86_64; test minimum compatible Rust and current stable, install GCC and bindgen prerequisites, and initialize the upstream submodule.
3. Run a short encode smoke step in CI and use ffprobe to inspect codec, profile, dimensions, and frame count. Full FFmpeg decoding can be added later.
4. Document installation/build prerequisites, crate roles, YUV420P input, examples, and troubleshooting in rustdoc and project docs.
5. Record upstream attribution and licenses, and ensure packaging includes the intended source artifacts.
6. Keep the project git-only and document the submodule update review procedure.
7. Do not add benchmarks or a CLI for the initial milestone.

## Acceptance

- Unit tests and the ffprobe smoke check pass on GNU/Linux x86_64.
- Examples and build instructions work in a clean checkout.
- Licenses, package contents, and upstream revision are documented.
- Release process and first release criteria are explicit.
