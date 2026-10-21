# yah264-rs

**yah264-rs** is a safe wrapper written in Rust around **yah264** encoder (https://github.com/terranvigil/yah264).

## General git rules

Each crate will have its specific .gitignore file.

## Rust project structure

### yah264-sys crate

Unsafe wrapper crate around yah264.

### yah264 crate

Safe wrapper crate around yah264-sys crate.

### av-yah264 crate

Native implementation of yah264 encoder for the rust-av (https://github.com/rust-av/rust-av) ecosystem written in idiomatic Rust.

## bindgen and build.rs

A `build.rs` in yah264-sys crate with `cc` crate or `cmake` crate for compiling C/C++ source code in `vendor/yah264`.

## Memory Model

Where possible use Google zerocopy withing rust-av or yah264 crates.
