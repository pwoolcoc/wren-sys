use std::old_io::{fs, Command};
use std::os;
use std::old_io::process::InheritFd;

fn main() {
    let manifest_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
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

    println!("cargo:rustc-flags=-L {} -l wren:static", wren_dir.display());
}
