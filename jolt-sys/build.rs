use normpath::PathExt;
use std::path::{Path, PathBuf};

fn main() {
    // Rerun on changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=jolt-physics");
    println!("cargo:rerun-if-changed=jolt-wrapper");

    // Get needed cargo options.
    let out_dir =
        PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR must be set in build scripts."));
    let opt_level = std::env::var("OPT_LEVEL").expect("OPT_LEVEL must be set in build scripts.");

    // Build up various needed paths.
    let jolt_base_path = Path::new("./jolt-physics");
    let jolt_include_path = jolt_base_path.clone();
    let jolt_build_path = jolt_base_path.join("Build");
    let jolt_out_path = out_dir.join("jolt-physics");
    let _ = std::fs::create_dir(&jolt_out_path);

    // Compile Jolt.
    let jolt_binary_path = compile_jolt(&opt_level, &jolt_build_path, &jolt_out_path);
    let jolt_binary_path = jolt_binary_path.join(if opt_level == "0" {
        "build/Debug"
    } else {
        "build/Release"
    });

    // Generate FFI.
    generate_ffi(&jolt_include_path);

    // Compile wrapper.
    let wrapper_base_path = Path::new("./jolt-wrapper");
    let wrapper_include_path = wrapper_base_path.join("inc");
    let wrapper_build_path = wrapper_base_path.clone();
    let wrapper_out_path = out_dir.join("wrapper");
    let _ = std::fs::create_dir(&wrapper_out_path);

    let wrapper_binary_path = compile_wrapper(
        &opt_level,
        &wrapper_build_path,
        &wrapper_out_path,
        &jolt_include_path,
    );
    let wrapper_binary_path = wrapper_binary_path.join(if opt_level == "0" {
        "build/Debug"
    } else {
        "build/Release"
    });

    // Set the include paths.
    println!("cargo:include={}", jolt_include_path.display());
    println!("cargo:include={}", wrapper_include_path.display());

    // Set the link search path.
    println!("cargo:rustc-link-search={}", jolt_binary_path.display());
    println!("cargo:rustc-link-search={}", wrapper_binary_path.display());

    // And link the library.
    println!("cargo:rustc-link-lib=static=Jolt");
    println!("cargo:rustc-link-lib=static=jolt-wrapper");

    // Add OS required libraries.
    println!("cargo:rustc-link-lib=libvcruntimed");
    println!("cargo:rustc-link-lib=ucrtd");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=userenv");
    println!("cargo:rustc-link-lib=shell32");
}

fn compile_jolt(opt_level: &str, build_path: &Path, out_path: &Path) -> PathBuf {
    let mut config = cmake::Config::new(build_path);
    config.generator("Visual Studio 16 2019");
    config.always_configure(true);
    if opt_level == "0" {
        config.profile("Debug");
    } else {
        config.profile("Release");
    }
    config.define("USE_SSE4_2", "ON");
    config.define("TARGET_UNIT_TESTS", "OFF");
    config.define("TARGET_HELLO_WORLD", "OFF");
    config.define("TARGET_PERFORMANCE_TEST", "OFF");
    config.define("TARGET_WINDOWS_ONLY", "OFF");
    config.out_dir(out_path);
    config.build_target("Jolt").build()
}

fn compile_wrapper(
    opt_level: &str,
    build_path: &Path,
    out_path: &Path,
    jolt_include_path: &Path,
) -> PathBuf {
    let mut config = cmake::Config::new(build_path);
    config.always_configure(true);
    config.generator("Visual Studio 16 2019");
    if opt_level == "0" {
        config.profile("Debug");
    } else {
        config.profile("Release");
    }
    config.define(
        "JOLT_INCLUDE_PATH",
        format!(
            "{}",
            jolt_include_path
                .normalize()
                .expect("Path must exist.")
                .as_path()
                .display()
        ),
    );
    config.out_dir(&out_path);
    config.build_target("jolt-wrapper").build()
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
                &format!("-I{}/Jolt", includes.display()),
            ]
            .into_iter(),
        )
        .rustfmt_bindings(true)
        .generate_comments(false)
        .opaque_type("std::*")
        .allowlist_var("");

    // List the allowed types for generation.
    // This is unfortunately about the only thing we gain via bindgen.
    // The usage of STL+C++ API is overwhelming things no matter how
    // I approach it.
    let allowed_types = [("JPH", ["BroadPhaseLayer"])];
    for namespace in allowed_types {
        for type_name in namespace.1 {
            bindings = bindings.allowlist_type(String::from(namespace.0) + "::" + type_name);
        }
    }

    // List the headers we intend to generate bindings for.
    // Just the headers from hello world for now.
    let headers = [
        "Jolt/Jolt.h",
        "Jolt/Core/TempAllocator.h",
        "Jolt/Core/JobSystemThreadPool.h",
        "Jolt/Physics/PhysicsSettings.h",
        "Jolt/Physics/PhysicsSystem.h",
        "Jolt/Physics/Collision/Shape/BoxShape.h",
        "Jolt/Physics/Collision/Shape/SphereShape.h",
        "Jolt/Physics/Body/BodyCreationSettings.h",
        "Jolt/Physics/Body/BodyActivationListener.h",
    ];
    // Add the headers.
    for header in headers {
        bindings = bindings.header(includes.join(header).to_str().unwrap());
    }

    let bindings = bindings.generate().expect("Unable to generate jolt types.");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("jolt.rs"))
        .expect("Could not write bindings!");
}
