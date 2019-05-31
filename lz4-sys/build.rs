extern crate cc;

use std::env;

fn build_vendored() {
    let mut compiler = cc::Build::new();
    compiler
        .file("liblz4/lib/lz4.c")
        .file("liblz4/lib/lz4frame.c")
        .file("liblz4/lib/lz4hc.c")
        .file("liblz4/lib/xxhash.c")
        // We always compile the C with optimization, because otherwise it is 20x slower.
        .opt_level(3);
    match env::var("TARGET").unwrap().as_str() {
        "i686-pc-windows-gnu" => {
            compiler.flag("-fno-tree-vectorize");
        }
        _ => {}
    }
    compiler.compile("liblz4.a");
}

fn main() {
    let k = "LZ4_SYS_DELEGATE_LINKING";

    println!("cargo:rerun-if-env-changed={}", k);

    if env::var(k).is_ok() {
        return;
    }

    build_vendored();
}
