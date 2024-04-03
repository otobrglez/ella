use std::path::PathBuf;
use std::process::Command;

fn main() {
    let libdir_path = PathBuf::from("libprom2json")
        .canonicalize()
        .expect("cannot canonicalize path");

    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-v")
        //.arg("--ldflags")
        //.arg("'-linkmode external -extldflags=-static'")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(libdir_path.join("libprom2json.a"))
        .arg("libprom2json/main.go");

    go_build.status().expect("Go build failed");

    // TODO: This is disabled for now.
    // println!("cargo:rerun-if-changed=libprom2json/libprom2json.h");
    println!(
        "cargo:rustc-link-search=native={}",
        libdir_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=prom2json");

    let bindings = bindgen::Builder::default()
        .header("libprom2json/libprom2json.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("libprom2json").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
