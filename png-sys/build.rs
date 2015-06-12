use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::process::Stdio;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let is_target_embedded = target.find("eabi").is_some();

    if is_target_embedded {
        let cc = format!("{}-gcc", target);
        let ar = format!("{}-ar", target);
        let ranlib = format!("{}-ranlib", target);
        env::set_var("CC", &cc);
        env::set_var("AR", &ar);
        env::set_var("RANLIB", &ranlib);
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
        if is_target_embedded {
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
    println!("cargo:rustc-flags=-l png16:static -L {}/.libs", dst.display());
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
