//! rust-av encoder adapter for the safe yah264 API.

extern crate yah264_sys as ffi;
extern crate av_codec as codec;
extern crate av_data as data;

pub mod yah264_common;
pub mod yah264_encoder;
