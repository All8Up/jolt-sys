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
    let mut config = cmake::Config::new(build_path);
    config.generator("Visual Studio 16 2019");
    if opt_level == "0" {
        config.profile("Debug");
    } else {
        config.profile("Release");
    }
    config.define("USE_SSE4_2", "ON");
    config.static_crt(true).build_target("Jolt").build()
}

fn generate_ffi(includes: &Path) {
    // Generate the required FFI files from the Jolt headers.
    let mut bindings = bindgen::Builder::default()
        .clang_args(
            [
                "-x",
                "c++",
                "-std=c++17",
                "-msse4.2",
                "-mpopcnt",
                &format!("-I{}", includes.display()),
            ]
            .into_iter(),
        )
        .rustfmt_bindings(true)
        .generate_comments(false)
        .opaque_type("std::*")
        .allowlist_var("");

    // List the allowed types for generation.
    let allowed_types = [(
        "JPH",
        [
            "ContactListener",
            "BroadPhaselayer",
            "BroadPhaseLayerInterface",
            "BodyActivationListener",
        ],
    )];
    for namespace in allowed_types {
        for type_name in namespace.1 {
            bindings = bindings.allowlist_type(String::from(namespace.0) + "::" + type_name);
        }
    }

    // List the allowed functions for generation.
    let allowed_functions = [
        ("JPH", ["RegisterTypes"]),
        ("JPH", ["*"]),
    ];
    for namespace in allowed_functions {
        for func_name in namespace.1 {
            bindings = bindings.allowlist_function(String::from(namespace.0) + "::" + func_name);
        }
    }

    // List the headers we intend to generate bindings for.
    let headers = [
        "Jolt.h",
        "Core/TempAllocator.h",
        "Core/JobSystemThreadPool.h",
        "Physics/PhysicsSettings.h",
        "Physics/PhysicsSystem.h",
        "Physics/Collision/Shape/BoxShape.h",
        "Physics/Collision/Shape/SphereShape.h",
        "Physics/Body/BodyCreationSettings.h",
        "Physics/Body/BodyActivationListener.h",
    ];
    // Add the headers.
    for header in headers {
        bindings = bindings.header(includes.join(header).to_str().unwrap());
    }

    let bindings = bindings
        .generate()
        .expect("Unable to generate ittnotify bindings.");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("jolt.rs"))
        .expect("Could not write bindings!");
}
