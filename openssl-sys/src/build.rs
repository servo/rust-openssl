extern crate "pkg-config" as pkg_config;

use std::io::Command;
use std::io::process::StdioContainer::InheritFd;
use std::os;

fn main() {
    let mut flags = " -l crypto -l ssl".to_string();

    let target = os::getenv("TARGET").unwrap();

    // Android doesn't have libcrypto/libssl,
    // the toplevel Rust program should compile it themselves
    if target.find_str("android").is_some() {
        let in_dir = os::getenv("CARGO_MANIFEST_DIR").unwrap();

        Command::new("make").arg("-f")
                            .arg("makefile.android")
                            .stdin(InheritFd(0))
                            .stdout(InheritFd(1))
                            .stderr(InheritFd(2))
                            .status().unwrap();

        let path = format!("{}/openssl-1.0.1j", in_dir);
        os::setenv("OPENSSL_PATH", path.as_slice());

        flags.push_str(format!(" -L {}", path).as_slice());
        println!("cargo:rustc-flags={}", flags);
        return;
    }

    if pkg_config::find_library("openssl").is_err() {

        let win_pos = target.find_str("windows")
                            .or(target.find_str("win32"))
                            .or(target.find_str("win64"));

        // It's fun, but it looks like win32 and win64 both
        // have all the libs with 32 sufix
        if win_pos.is_some() {
           flags.push_str(" -l gdi32 -l wsock32");
        }



        println!("cargo:rustc-flags={}", flags);
    }
}
