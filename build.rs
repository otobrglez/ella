use std::path::PathBuf;
use std::process::Command;
use bindgen::CargoCallbacks;

fn main() {
    let libdir_path = PathBuf::from("libprom2json")
        .canonicalize()
        .expect("cannot canonicalize path");

    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(libdir_path.join("libprom2json.a"))
        .arg("libprom2json/main.go");

    go_build.status().expect("Go build failed");

    println!("cargo:rerun-if-changed={}", "libprom2json/main.go");
    println!("cargo:rustc-link-search=native={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=prom2json");

    let bindings = bindgen::Builder::default()
        .header("libprom2json/libprom2json.h")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("libprom2json").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

}

/*
fn main() {
    let libdir_path = PathBuf::from("libprom2json")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("prom2json.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the static library file.
    let obj_path = libdir_path.join("prom2json.o");
    let lib_path = libdir_path.join("prom2json.a");

    println!("cargo:rustc-link-lib=prom2json");

    /*
    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("hello.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }
    */

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("libprom2json").join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
*/
/*
fn main() {
    let out_path = PathBuf::from("./libprom2json");

    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_path.join("prom2json.a"))
        .arg("./libprom2json/main.go");

    go_build.status().expect("Go build failed");

    let bindings = bindgen::Builder::default()
        .header(out_path.join("prom2json.h").to_str().unwrap())
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=libprom2json/main.go");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static={}", "prom2json");
    // println!("cargo:rustc-link-lib=static={}", "prom2json");


}
*/

/*
fn main() {
    // println!("cargo:rustc-link-search=./");
    //println!("cargo:rustc-link-lib=static={}", "libprom2json/libprom2json.a");
    //println!("cargo:rustc-link-lib=libprom2json");
    // println!("cargo:rustc-link-search=./libprom2json");
    // println!("cargo:rustc-link-lib=prom2json");
    //println!("cargo:rustc-link-search=native={}", "./libprom2json");
    //println!("cargo:rustc-link-lib=static=prom2json"); // the name of the library
    //println!("cargo:rustc-link-search=native={}", "./libprom2json");
    //println!("cargo:rustc-link-lib=static={}", "libprom2json");

    let libdir_path = PathBuf::from("libprom2json")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("prom2json.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");
    let obj_path = libdir_path.join("prom2json.o");
    // let lib_path = libdir_path.join("prom2json.a");

    // libdir_path.to_str().unwrap()
    // println!("cargo:rustc-link-search=native={}", build_dir);


   // println!("cargo:rustc-link-arg=-mmacosx-version-min=12.0");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    // println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup,--unresolved-symbols=ignore-in-object-files");

    // println!("cargo:rustc-cdylib-link-arg=-Wl");
    // println!("cargo:rustc-cdylib-link-arg=--allow-multiple-definition");
    // println!("cargo:rustc-cdylib-link-arg=-undefined");
    // println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");

    println!("cargo:rustc-flags=-l framework=CoreFoundation -l framework=Security -l framework=SystemConfiguration");


    /*
    println!("cargo:rustc-link-arg=-Wl");
    println!("cargo:rustc-link-arg=-undefined");
    println!("cargo:rustc-link-arg=dynamic_lookup");
    */

    // println!("cargo:rustc-link-lib=prom2json");

    //println!("cargo:rustc-cdylib-link-arg=-undefined");
    //println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");

    // println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");

    //#[cfg(target_os = "macos")]
    //{

    // println!("cargo:rustc-cdylib-link-arg=-undefined");
    // println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    // println!("cargo:rustc-cdylib-link-arg=-Wl,-undefined,dynamic_lookup");

    // println!("cargo:rustc-cdylib-link-arg=-Wl,--no-as-needed,-ldl,--as-needed");

    // println!("cargo:rustc-link-lib=static=c++");
    // println!("cargo:rustc-cdylib-link-arg=-Wl,--allow-multiple-definition");




    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("libprom2json");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
*/
