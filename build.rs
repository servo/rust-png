extern crate gcc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg: gcc::Config = gcc::Config::new();

    cfg.file("src/shim.c");

    let src_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("png-sys/libpng-1.6.16");
    cfg.include(&src_dir);

    let dep_dir = PathBuf::from(&env::var("DEP_PNG_ROOT").unwrap());
    cfg.include(&dep_dir);

    cfg.compile("libpngshim.a");
}
