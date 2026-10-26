extern crate regex;
extern crate yah264;

use yah264::api::low_level_api::{Encoder, Param, Picture};

use regex::Regex;
use yah264_sys::yah264_csp_t_YAH264_CSP_I420;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"(\d+)x(\d+)").unwrap();
    if args.len() < 3 {
        panic!(
            "Missing argument:\nUsage:\n{} 1920x1080 in.yuv out.h264\n",
            args[0],
        )
    }
    let caps = re.captures(args[1].as_str()).unwrap();
    let w: usize = caps[1].parse().unwrap();
    let h: usize = caps[2].parse().unwrap();

    let mut par = Param::new();
    par = par.default_preset("medium", "none").unwrap();
    par = par.set_dimension(h, w);
    par = par.set_colorspace(yah264_csp_t_YAH264_CSP_I420 as i32);
    
    let mut pic = Picture::from_param(&par).unwrap();

    let mut enc = Encoder::open(&mut par).unwrap();
    let mut input = File::open(args[2].as_str()).unwrap();
    let mut output = File::create(args[3].as_str()).unwrap();
    let mut timestamp: i64 = 0;

    let headers = enc.get_headers().unwrap();
    output.write_all(headers.as_bytes()).unwrap();

    'out: loop {
        // TODO: read by line, the stride could be different from width
        for plane in 0..3 {
            let buf = pic.as_mut_slice(plane).unwrap();
            if input.read_exact(buf).is_err() {
                break 'out;
            }
        }

        pic = pic.set_timestamp(timestamp);
        timestamp += 1;
        if let Some((nal, _)) = enc.encode(&pic).unwrap() {
            let buf = nal.as_bytes();
            output.write_all(buf).unwrap();
        }
    }

    // Keep invoke encode(None) until the encoder returns Ok(None)
    loop {
        match enc.encode(None) {
            Ok(Some((nal, _))) => {
                let buf = nal.as_bytes();
                output.write_all(buf).unwrap();
            },
            Ok(None) => {
                break;
            },
            Err(e) => {
                eprintln!("Error during flushing: {}", e);
                break;
            },
        }
    }
}
