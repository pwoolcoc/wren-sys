use std::io::{fs, Command};
use std::os;
use std::io::process::InheritFd;

fn main() {
    let manifest_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());
    let wren_dir = manifest_dir.join(Path::new("src/wren"));
    let wren_lib = manifest_dir.join(Path::new("src/wren/libwren.a"));

    let mut make = Command::new("make");

    assert!(make.cwd(&wren_dir)
                .arg("release")
                .stdout(InheritFd(1))
                .stderr(InheritFd(2))
                .status()
                .unwrap()
                .success());

    if let Err(_) = fs::copy(&wren_lib, &out_dir.join("libwren.a")) {
        println!("ERROR COPYING libwren.a");
    }

    println!("cargo:rustc-flags=-L {} -l wren:static", out_dir.display());
}
