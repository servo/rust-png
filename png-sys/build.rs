use std::io::Command;
use std::io::process::InheritFd;
use std::os;

fn main() {
    let cfg = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("libpng-1.6.16/configure");
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    os::setenv("CFLAGS", "-fPIC -O3");

    let mut cmd = Command::new(cfg);
    cmd.arg("--with-libpng-prefix=RUST_");
    cmd.cwd(&dst);
    run(&mut cmd);

    let mut cmd = Command::new("make");
    cmd.arg("-j4");
    cmd.cwd(&dst);
    run(&mut cmd);

    println!("cargo:root={}", dst.display());
    println!("cargo:rustc-flags=-l png16:static -L {}/.libs", dst.display());
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());
}
