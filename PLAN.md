# yah264-rs Project Plan

**Status:** In progress — upstream submodule and workspace/build skeleton are in place  
**Scope:** Three-crate workspace; first milestone is an 8-bit H.264 encoder usable through rust-av.

## Goal

Build a Rust project around the upstream yah264 H.264 encoder with three distinct layers:

1. `yah264-sys`: a small, unsafe FFI layer over the upstream native implementation.
2. `yah264`: a safe Rust wrapper over `yah264-sys`.
3. `av-yah264`: an adapter from the safe `yah264` wrapper to the `Encoder` trait in rust-av's codec crate.

Use the upstream source as a mandatory git submodule at `vendor/yah264`, tracking upstream `main` and treated as read-only. Build it with `cc` in `yah264-sys/build.rs`, generate bindings with bindgen, and use Google `zerocopy` in `yah264` to avoid copies wherever the representation and ownership allow it safely.

## Current state and constraints

- The repository currently contains project guidance and this plan; there are no manifests, source files, or CI configuration.
- The workspace will contain `yah264-sys`, `yah264`, and `av-yah264`; distribution is git-only for now.
- Initial target: GNU/Linux x86_64 using the minimum GCC version needed by upstream. Build a static archive only.
- The upstream submodule must track `main`; the git maintenance role described by `.agents/git.md` owns its updates. Do not patch upstream locally.
- Original Rust code is dual Apache-2.0/MIT. Upstream yah264 is GPL-2.0-or-later; its license is separate, and distribution of binaries that include it needs review.
- The checked upstream revision is `8b5306358e364c005c09d43e47ab92f9d39aaf90` (2026-10-20). The checked rust-av revision is `d44bb083ffc543d80397c8244f2e8c534f609308`.
- yah264 returns encoder-owned NAL buffers valid only until its next API call. rust-av's current `Packet` owns a `Vec<u8>`, so its adapter must copy encoded bytes; frame planes can be borrowed during `send_frame`.
- Upstream's canonical build uses Meson, C11, pthreads, and libm. The Rust build will compile its scalar C source set through `cc`; optional SIMD tiers are not included initially.
- Add unit tests initially. CI also runs a short encode smoke step and uses ffprobe to inspect codec, profile, dimensions, and frame count.
- No hard latency, throughput, or memory targets are set. Encode a short YUV420P sequence at a modest even resolution with upstream rate-control defaults.
- Crate-specific `.gitignore` files should follow the applicable Rust, C, and C++ ignore patterns from GitHub's gitignore collection.
- Confirmed design details and implementation checks are in `.plans/open-questions.md`.

## Guiding decisions

- Keep the unsafe FFI boundary narrow and place the ergonomic, safe API in `yah264`.
- Keep the sys crate responsible for native compilation and bindings; avoid leaking native representation into safe public APIs.
- Treat upstream yah264 as the authoritative encoder implementation. The submodule follows `main`, while build/test output records the checked-out commit for reproducibility.
- Implement the rust-av `Encoder` trait in `av-yah264`, adapting the safe wrapper.
- Support 8-bit YUV420P input first. Callers provide encoder-ready frames; conversion is outside the initial scope.
- Accept three YUV420P plane slices with per-plane strides, validate lengths, require even dimensions initially, and pass color metadata through only where both APIs support it.
- Expose dimensions and settings required by rust-av/upstream; use upstream defaults for other encoder settings.
- Expose yah264 NAL data as a borrow tied to the encoder because upstream invalidates it on the next API call. `av-yah264` copies bytes into rust-av's owned `Packet.data` vector; input YUV planes are borrowed directly for encoding.
- Encode incrementally without a hard latency/throughput promise or a required callback API.
- Return typed errors with native status codes; require exclusive mutable access to each encoder. Add `Send` only if upstream behavior permits; do not add `Sync` without explicit support.
- Require safety comments for unsafe blocks, `# Safety` documentation for public unsafe functions, and deny `unsafe_op_in_unsafe_fn`; safe public APIs contain no `unsafe` unless a concrete need arises.
- Meet the zero-copy requirement wherever buffer ownership, layout, and lifetimes make it possible. Record and explain unavoidable copies.
- Choose the lowest Rust edition/MSRV compatible with the selected rust-av and build dependencies; there is no separate compatibility promise yet.
- Keep the initial feature set to the simplest upstream-supported H.264 configuration.

## Work plan

### 0. Confirm requirements and upstream constraints

- Perform the source/API checks in `.plans/open-questions.md`.
- Inspect yah264 `main`: source layout, build instructions, exported API, packet ownership, dependencies, license, supported platforms, and threading behavior.
- Inspect and pin the rust-av codec crate revision; verify the `Encoder` trait's frame, packet, and flush semantics.
- Record decisions that affect ABI, crate names, feature flags, build tooling, supported targets, and compatibility in a short decision log.

**Exit criteria:** upstream build and license are understood; target and rust-av revision are recorded; the implementation assumptions for frame/output/config/threading match both upstream APIs.

**Progress:** upstream and rust-av revisions are pinned for development, and their current API shapes are recorded. The upstream GPL license still needs a redistribution review.

### 1. Establish repository and workspace

- Create the Cargo workspace and the three crate directories named `yah264-sys`, `yah264`, and `av-yah264` with the agreed dependency direction.
- Add crate-level `.gitignore` files based on the Rust, C, and C++ templates specified by the user; review for relevant patterns.
- Add Apache-2.0 and MIT license files after confirming upstream license compatibility.
- Add `vendor/yah264` as a submodule tracking upstream `main`; document initialization and the `.agents/git.md` update procedure.
- Add top-level documentation describing the crates and prerequisites.

**Exit criteria:** a clean checkout can initialize the submodule and Cargo recognizes all three crates; workspace edition/MSRV is the lowest compatible choice.

**Progress:** the `vendor/yah264` submodule, root Cargo workspace, three crate manifests, per-crate ignore files, README, and initial `yah264-sys/build.rs` are in place. License texts and the minimum compiler/MSRV are still to be finalized.

### 2. Implement `yah264-sys`

- Build upstream's scalar C sources into a static archive with the `cc` crate and generate bindings with bindgen; install/use clang/libclang as documented prerequisites.
- Produce a static archive as requested and verify upstream can be built through `cc`.
- Expose only the native ABI needed by the safe wrapper.
- Document ownership, pointer validity, lengths, alignment, callbacks, and native error/status behavior at each unsafe boundary.
- Define supported target and feature behavior, including clear build errors for unsupported configurations.

**Exit criteria:** the sys crate builds the selected upstream revision on GNU/Linux x86_64 with the minimum required GCC and its generated bindings match the native ABI.

### 3. Implement the safe `yah264` API

- Design Rust types for the simplest upstream-supported 8-bit H.264 configuration, YUV420P input frames, encoded output, and errors.
- Map every raw pointer and native resource to a Rust ownership model with explicit creation, use, and cleanup behavior.
- Validate even frame dimensions, Y/U/V plane lengths, per-plane strides, and configuration before entering native code.
- Expose encoder-owned NAL bytes through a borrow that prevents another encoder call while the bytes are in use; document the upstream lifetime.
- Encode sequences incrementally; don't add a callback requirement or latency guarantee unless a concrete rust-av use case requires it.
- Require exclusive mutable access; establish `Send` only if upstream semantics allow it and avoid `Sync` without explicit native guarantees.
- Use `zerocopy` in the safe wrapper where data layout and lifetimes are verified; document any unavoidable copies.

**Exit criteria:** normal safe use requires no caller-written `unsafe`; callers can submit encoder-ready YUV420P frames; invalid inputs produce typed Rust errors; native resources are released on all paths.

### 4. Integrate with rust-av (`av-yah264`)

- Pin the rust-av revision and use its codec crate's `Encoder` trait.
- Implement that trait as an adapter over `yah264`, mapping YUV420P frames, timestamps, configuration, output packets, and errors.
- Reuse the safe wrapper and its zero-copy paths where the trait's buffer ownership permits.
- Document unsupported format or feature combinations and any required conversions.

**Exit criteria:** the crate implements rust-av's selected `Encoder` trait and can encode an 8-bit H.264 frame from YUV420P input.

### 5. Validation, documentation, and delivery

- Add unit tests first, structured so FFI and end-to-end coverage can be added later.
- Add GitHub Actions CI on Ubuntu x86_64 with explicit submodule checkout, native prerequisites, minimum compatible Rust, and current stable Rust.
- Have CI encode a short sample and use ffprobe to inspect codec, profile, dimensions, and frame count. Full FFmpeg decoding is not required initially.
- Add rustdoc and build documentation, following rust-av project consensus where relevant; include upstream revision and license attribution.
- Do not add benchmarks or CLI tooling in the initial milestone.
- Define git-only versioning and submodule update procedure.

**Exit criteria:** unit tests and ffprobe smoke validation pass; the crate implements the pinned rust-av encoder trait; a clean GNU/Linux x86_64 build produces a valid 8-bit H.264 bitstream from a short YUV420P sequence.

## Risks and dependencies

- Upstream `main` can change ABI or build requirements. Keep the submodule on `main` as requested, while recording each tested commit and reviewing updates.
- Upstream yah264 files are GPL-2.0-or-later while this project's original Rust code is intended to be dual MIT/Apache-2.0. Preserve upstream notices and review the combined static-binary distribution obligations before distributing binaries.
- Bindgen, clang/libclang, and GCC are build prerequisites; document versions and failures clearly. Build scripts must not download resources.
- Encoder APIs often retain buffers or invoke callbacks. The safe API must reflect the native lifetime and threading rules rather than assume synchronous consumption.
- rust-av APIs may evolve. Pin the integration target and make compatibility policy explicit.
- Requiring zero-copy wherever possible can constrain packet ownership and lifetimes. Check rust-av's output ownership requirements before finalizing the safe API; retain a documented copy fallback only where required.

## Implementation checks

Planning questions are answered. Before implementation, verify:

1. Complete the safe wrapper and packet/frame mapping against the inspected rust-av trait.
2. Confirm threading and default configuration behavior in yah264's implementation.
3. Review upstream license obligations before distributing combined binaries.
4. Establish the minimum compatible GCC and Rust versions.

## Plan documents

- [`.plans/README.md`](.plans/README.md): index and status.
- [`.plans/01-foundation.md`](.plans/01-foundation.md): requirements, workspace, and upstream setup.
- [`.plans/02-ffi.md`](.plans/02-ffi.md): native build and `yah264-sys`.
- [`.plans/03-safe-api.md`](.plans/03-safe-api.md): safe wrapper and memory model.
- [`.plans/04-rust-av.md`](.plans/04-rust-av.md): rust-av integration.
- [`.plans/05-validation-release.md`](.plans/05-validation-release.md): validation and delivery.
- [`.plans/open-questions.md`](.plans/open-questions.md): confirmed decisions and implementation checks.
