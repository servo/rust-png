use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::process::Stdio;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    if host != target {
        if env::var_os("CC").is_none() {
            env::set_var("CC", format!("{}-gcc", target));
        }
        if env::var_os("AR").is_none() {
            env::set_var("AR", format!("{}-ar", target));
        }
        if env::var_os("RANLIB").is_none() {
            env::set_var("RANLIB", format!("{}-ranlib", target));
        }
    }

    let cfg = PathBuf::from(&env::var("CARGO_MANIFEST_DIR")
                            .unwrap()).join("libpng-1.6.16/configure");
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());
    env::set_var("CFLAGS", "-fPIC -O3");
    let mut cmd;
    if host.find("windows").is_some() {
        // Here be windows hacks

        // Windows doesn't know how to handle executables
        // with shebangs, so we pass it through `sh`
        cmd = Command::new(PathBuf::from("sh"));

        // Windows under mingw also doesn't understand its
        // own paths, so we pass the path through cygpath
        cmd.arg("-c").arg(&format!("$(cygpath \"{}\") --with-libpng-prefix=RUST_",
                                   cfg.to_str().unwrap()));
    } else {
        cmd = Command::new(cfg);
        cmd.arg("--with-libpng-prefix=RUST_");
        if host != target {
            cmd.arg(format!("--host={}", target));
        }
    }
    cmd.current_dir(&dst);
    run(&mut cmd);
    let mut cmd = Command::new("make");
    cmd.arg("-j4");
    cmd.current_dir(&dst);
    run(&mut cmd);

    println!("cargo:root={}", dst.display());
    println!("cargo:rustc-link-search=native={}/.libs", dst.display());
    println!("cargo:rustc-link-lib=static=png16");
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
