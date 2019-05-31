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
    match env::var("TARGET").unwrap().as_str()
    {
        "i686-pc-windows-gnu" => {
            compiler
                .flag("-fno-tree-vectorize");
        },
        _ => {}
    }
    compiler.compile("liblz4.a");
}

fn use_lib(dir: String) {
    println!("cargo:warning=using liblz4 from: {}", dir);
    println!("cargo:rustc-link-search=native={}/lib", dir);
    println!("cargo:include=native={}/include", dir);
    println!("cargo:rustc-link-lib=lz4");
}

fn main() {
    println!("cargo:rerun-if-env-changed=LIBLZ4_DIR");

    match env::var("LIBLZ4_DIR") {
        Ok(dir) => use_lib(dir),
        Err(_) => build_vendored(),
    }
}
