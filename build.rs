extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libpngshim.a",
                         &Default::default(),
                         &["src/shim.c"]);
}
