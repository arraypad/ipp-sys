use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn thin_universal_binary(universal_fname: &PathBuf, thin_fname: &PathBuf, arch: &str) -> PathBuf {
    let thin_dir_str = env::var("OUT_DIR").expect("getting OUT_DIR");
    let thin_dir = Path::new(&thin_dir_str);
    Command::new("lipo")
        .arg(universal_fname.as_os_str())
        .arg("-thin")
        .arg(arch)
        .arg("-output")
        .arg(thin_fname)
        .current_dir(&thin_dir)
        .status()
        .expect("running lipo");
    thin_dir.to_path_buf()
}

pub fn ipp_build(libname: &str) {
    let ipproot = match env::var("IPPROOT") {
        Ok(dir) => dir,
        Err(e) => {
            panic!("Environment variable IPPROOT could not be read: {}", e);
        }
    };

    let libname: String = libname.to_string();
    let ipp_path = Path::new(&ipproot);
    let libdir_base = ipp_path.join("lib");

    let link_type = match env::var_os("IPP_STATIC") {
        Some(v) => if &v == "0" { "dylib" } else { "static" },
        None => "dylib",
    };

    let target = env::var("TARGET").expect("getting target");
    ipp_build_inner(libname, libdir_base, target, link_type)
}

fn ipp_build_inner(libname: String, libdir_base: PathBuf, target: String, link_type: &str) {
    assert!(link_type == "static" || link_type == "dylib");
    let mut libdir = libdir_base.clone();
    match target.as_ref() {
        "x86_64-apple-darwin" => {
            if link_type == "static" {
                let lib_filename = "lib".to_string() + &libname.to_string() + ".a";
                let universal_fname = libdir.join(lib_filename.clone());
                let thin_fname = lib_filename.clone();
                libdir = thin_universal_binary(&universal_fname, &thin_fname.into(), "x86_64");
            } else {
                libdir = libdir.join("intel64");
            }
        }
        "x86_64-unknown-linux-gnu" | "x86_64-unknown-linux-musl" => {
            libdir = libdir.join("intel64");
        }
        "x86_64-pc-windows-msvc" => {
            libdir = libdir.join("intel64");
        }
        t => {
            panic!("No ipp build support implemented for target {}", t);
        }
    }

    println!("cargo:rustc-link-search={}", libdir.display());
    println!("cargo:rustc-link-lib={}={}", link_type, libname);
}
