# 01 — Foundation and upstream setup

## Outcomes

- Three-crate workspace: `yah264-sys`, `yah264`, and `av-yah264`.
- Initial target and acceptance criteria: an 8-bit H.264 frame encoded through rust-av on GNU/Linux x86_64.
- Cargo workspace containing `yah264-sys`, `yah264`, and `av-yah264`.
- Reproducible upstream source checkout under `vendor/yah264`.
- Repository metadata and crate-local ignore rules.

## Tasks

1. Inspect rust-av and pin a revision; derive the edition/MSRV floor from selected dependencies; verify upstream license compatibility as listed in `open-questions.md`.
2. Review the upstream repository's source structure, license, build system, ABI, target support, and update history.
3. Add upstream yah264 as a mandatory submodule tracking `main`; keep it read-only and document initialization/update steps.
4. Create the workspace and crate manifests with only the dependencies justified by the agreed design.
5. Add per-crate `.gitignore` files based on the Rust, C, and C++ templates requested in `CONTEXT.md` and by the user.
6. Record foundational decisions and target compatibility in project documentation.

## Acceptance

- A new checkout can initialize the intended upstream revision.
- Cargo discovers all workspace members.
- Package metadata and licensing are consistent with upstream distribution requirements.
- Supported Rust and native toolchains are documented.
