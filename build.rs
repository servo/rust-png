#![feature(os, path)]

extern crate gcc;

use std::default::Default;
use std::os;
use std::path::PathBuf;

fn main() {
    let mut cfg: gcc::Config = gcc::Config::new();

    cfg.file("src/shim.c");

    let src_dir = PathBuf::new(&os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("png-sys/libpng-1.6.16");
    cfg.include(&src_dir);

    let dep_dir = PathBuf::new(&os::getenv("DEP_PNG_ROOT").unwrap());
    cfg.include(&dep_dir);

    cfg.compile("libpngshim.a")
}
