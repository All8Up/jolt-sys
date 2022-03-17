use std::path::{Path, PathBuf};

fn main() {
    // Get needed cargo options.
    let opt_level = std::env::var("OPT_LEVEL").expect("Cargo build scripts always have OPT_LEVEL");

    // Compile Jolt.
    let compiled_lib_path = compile_jolt(Path::new("./jolt/Build"), &opt_level);

    // Generate FFI.
    generate_ffi(Path::new("./jolt/Jolt"));

    // Adjust the library source path based on the opt level.
    let lib_out_path = if opt_level == "0" {
        compiled_lib_path.join("build/Debug")
    } else {
        compiled_lib_path.join("build/Release")
    };

    // Set the include paths.
    println!("cargo::include={}", "./jolt/Jolt");

    // Set the link search path.
    println!("cargo:rustc-link-search={}", lib_out_path.display());

    // And link the library.
    println!("cargo:rustc-link-lib=static=Jolt");
}

fn compile_jolt(build_path: &Path, opt_level: &str) -> PathBuf {
    // Other than forcing the static crt, just need to adjust the config
    // to match OPT_LEVEL.
    let mut config = cmake::Config::new(build_path);
    if opt_level == "0" {
        config.profile("Debug");
    } else {
        config.profile("Release");
    }
    config.static_crt(true).build_target("Jolt").build()
}

fn generate_ffi(includes: &Path) {
    // Generate the required FFI files from the Jolt headers.
    let bindings = bindgen::Builder::default()
        .clang_args(
            [
                "-x",
                "c++",
                "-std=c++17",
                &format!("-I{}", includes.display()),
            ]
            .into_iter(),
        )
        .rustfmt_bindings(true)
        .header(includes.join("Jolt.h").to_str().unwrap())
        .allowlist_type("")
        .allowlist_var("")
        .allowlist_function("")
        .generate()
        .expect("Unable to generate ittnotify bindings.");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("jolt_h.rs"))
        .expect("Could not write bindings!");
}
