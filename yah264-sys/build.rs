use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").expect("Cargo must set TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cargo must set target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Cargo must set target OS");
    if arch != "x86_64" || os != "linux" {
        panic!("yah264-sys currently supports GNU/Linux x86_64 only (got {target})");
    }

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().expect("crate is a workspace member");
    let upstream = repo_root.join("vendor/yah264");
    let include = upstream.join("include");
    let src = upstream.join("src");

    let sources = [
        "common/bitstream.c",
        "common/ledger.c",
        "common/stgprof.c",
        "common/iprof.c",
        "common/nal.c",
        "common/cpu.c",
        "common/threadpool.c",
        "dsp/metrics.c",
        "dsp/transform.c",
        "dsp/intrapred.c",
        "dsp/interp.c",
        "dsp/loopfilter.c",
        "encoder/params.c",
        "encoder/cavlc.c",
        "encoder/cabac.c",
        "encoder/motion.c",
        "encoder/mbcode.c",
        "encoder/skiporacle.c",
        "encoder/loopfilter.c",
        "encoder/paramsets.c",
        "encoder/sei.c",
        "encoder/encoder.c",
        "encoder/rate.c",
        "encoder/nr.c",
        "encoder/mcstf.c",
        "encoder/gpu.c",
        "hw/vt_stub.c",
        "hw/vt_map.c",
    ];

    let mut build = cc::Build::new();
    build
        .include(&include)
        .include(&src)
        .std("c11")
        .define("_GNU_SOURCE", None)
        .define("Y264_BIT_DEPTH", "8")
        .define("Y264_HAVE_NEON", "0")
        .define("Y264_HAVE_SVE", "0")
        .define("Y264_HAVE_SSE4", "0")
        .define("Y264_HAVE_AVX2", "0")
        .define("Y264_HAVE_AVX512", "0")
        .define("Y264_HAVE_VT", "0")
        .flag_if_supported("-pthread");
    for source in sources {
        build.file(src.join(source));
    }
    build.compile("yah264_native");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rerun-if-changed={}", include.join("yah264.h").display());
    println!("cargo:rerun-if-changed={}", upstream.join("meson.build").display());

    let bindings = bindgen::Builder::default()
        .header(include.join("yah264.h").to_string_lossy())
        .clang_arg(format!("-I{}", include.display()))
        .clang_arg("-DY264_BIT_DEPTH=8")
        .allowlist_function("yah264_.*")
        .allowlist_type("yah264_.*")
        .allowlist_var("YAH264_.*")
        .layout_tests(false)
        .generate()
        .expect("failed to generate yah264 bindings; install clang/libclang");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo must set OUT_DIR"));
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write yah264 bindings");
}
