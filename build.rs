#![feature(os, path)]

extern crate gcc;

use std::default::Default;
use std::os;

fn main() {
    let mut cfg: gcc::Config = Default::default();

    let src_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("png-sys/libpng-1.6.16");
    cfg.include_directories.push(src_dir);

    let dep_dir = Path::new(os::getenv("DEP_PNG_ROOT").unwrap());
    cfg.include_directories.push(dep_dir);

    gcc::compile_library("libpngshim.a",
                         &cfg,
                         &["src/shim.c"]);
}
