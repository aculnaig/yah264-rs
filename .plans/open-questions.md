# Confirmed decisions and implementation checks

The planning questions have been answered. This file records the decisions and the concrete source/API checks that remain before implementation. It is linked from the project plan in [PLAN.md](../PLAN.md).

## Confirmed decisions

| Area | Decision |
|---|---|
| Workspace | Three crates: `yah264-sys`, `yah264`, and `av-yah264`. |
| First milestone | Encode a short 8-bit H.264 sequence through `av-yah264` and rust-av. All three crates are in scope. |
| rust-av | Implement the codec crate's `Encoder` trait. Pin the rust-av revision; use a local path only when co-developing against a checkout. |
| Distribution | Git-only for now. |
| License | Dual Apache-2.0/MIT, subject to verifying upstream and native dependency compatibility. |
| Upstream source | Mandatory `vendor/yah264` submodule tracking upstream `main`; read-only locally. The git maintenance role described in `.agents/git.md` updates it. No other upstream version is targeted. |
| Native build | Use `cc` and bindgen; clang/libclang are documented prerequisites. Produce a static archive only; this resolves the earlier static/dynamic selection request in favor of static linking. |
| Initial target | GNU/Linux x86_64, with the minimum GCC version required by yah264. CI uses GitHub Actions on Ubuntu x86_64 and tests the minimum compatible Rust version plus current stable. |
| Encoder input | 8-bit YUV420P; callers provide encoder-ready frames. Accept three plane slices with per-plane strides, validate sizes, and initially require even dimensions. Pass color metadata through when both rust-av and yah264 support it; otherwise document defaults. No input conversion. |
| Encoder configuration | Expose dimensions and settings required by rust-av/upstream; use upstream defaults for other settings. |
| Encoded output | Prefer an owned native packet wrapped with Rust RAII and borrowed byte access. Copy only if upstream exposes temporary internal storage. Rust-av controls packet delivery; add callbacks only if the trait or concrete use case requires them. |
| Streaming and latency | Encode a sequence incrementally. No hard latency, frame-rate, or memory guarantee is set. No callback API is required by default. |
| Errors | Use typed Rust errors for invalid input, unsupported format/configuration, allocation failure, and native failure; retain the native status code. Return errors instead of panicking for invalid inputs. |
| Threading | Require exclusive mutable access to an encoder. Separate instances may encode concurrently. Implement `Send` only when upstream semantics support it; do not implement `Sync` absent explicit native guarantees. |
| Unsafe policy | Deny `unsafe_op_in_unsafe_fn`; document every unsafe block; require `# Safety` sections on public unsafe functions; expose no public unsafe API from the safe wrapper or adapter without a concrete need. |
| Memory | Use `zerocopy` in the safe wrapper and avoid copies wherever ownership, layout, and lifetime rules allow. Document unavoidable copies. |
| Validation | Add unit tests initially. Also run a CI smoke step that encodes a short sample and uses ffprobe to inspect codec, profile, dimensions, and frame count. Full FFmpeg decoding is not part of the initial default. |
| Dependencies | Allow explicitly declared Cargo dependencies. Build scripts must not download resources. Fetch yah264 only through the documented submodule steps. Support offline builds after Cargo dependencies and the submodule are present. |
| Workload sample | Use a short YUV420P sequence at a modest even resolution and upstream rate-control defaults. Do not promise a latency or throughput target. |
| Rust version | Select the lowest Rust edition and MSRV compatible with rust-av and all dependencies; document it. |
| Documentation | Rustdoc or rust-av project consensus. No CLI or benchmarks for now. |
| Ignore rules | Each crate gets its own `.gitignore`, based on the Rust, C, and C++ templates in GitHub's gitignore collection. |
| Practices | No additional coding, review, or release practices were specified. |

## Implementation checks

These are repository/API inspection tasks, not unanswered requirements:

1. **Inspected:** yah264 main is at `8b5306358e364c005c09d43e47ab92f9d39aaf90` (2026-10-20). The public API uses `yah264_param_default`, `yah264_encoder_open`, `yah264_encoder_encode`, and `yah264_encoder_close`. It accepts caller-owned 8-bit YUV420P planes with per-plane sample strides. NAL payload and descriptor storage belong to the encoder and remain valid only until its next call.
2. **Inspected:** rust-av main is at `d44bb083ffc543d80397c8244f2e8c534f609308`. Its `Encoder` trait is `Send`, takes `&ArcFrame`, and returns `av_data::packet::Packet`, whose `data` is an owned `Vec<u8>`. `ArcFrame` exposes immutable plane slices and strides. Input can be borrowed for the encode call; encoded bytes must be copied into rust-av packets before the next yah264 call.
3. **License review required before binary distribution:** yah264 files declare `GPL-2.0-or-later`; the project's original Rust code is intended to remain dual Apache-2.0/MIT. Preserve the upstream license and review the combined distribution obligations before distributing a statically linked binary.
4. **Build inspection:** upstream's canonical build is Meson, with C11, pthreads, and libm. The workspace build script is being set up to compile the scalar C sources into a static archive with `cc`; SIMD tiers are omitted initially. The exact minimum GCC version still needs to be established by source/build compatibility.
5. Derive workspace edition/MSRV from the lowest versions supported by rust-av, `zerocopy`, bindgen, `cc`, and other selected dependencies.
6. Base per-crate ignore files on the requested Rust, C, and C++ templates, trimming irrelevant patterns where appropriate.
