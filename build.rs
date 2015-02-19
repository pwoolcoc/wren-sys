use std::old_io::{fs, Command};
use std::os;
use std::old_io::process::InheritFd;

fn main() {
    let manifest_dir = Path::new(os::getenv("PWD").unwrap());
    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());
    let src_dir = manifest_dir.join("src");
    let wren_dir = src_dir.join("wren");
    let wren_lib = wren_dir.join("libwren.a");

    let mut make = Command::new("make");

    assert!(make.cwd(&Path::new("src/wren"))
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
