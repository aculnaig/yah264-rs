//! Safe Rust API for the yah264 H.264 encoder.

#![deny(unsafe_op_in_unsafe_fn)]

extern crate yah264_sys as ffi;

pub mod api;
