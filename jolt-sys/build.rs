use std::path::{Path, PathBuf};

#[cfg(feature = "bundled")]
extern crate cmake;

fn main() {
    #[cfg(feature = "bundled")]
    let _path = compile_jolt(Path::new("./jolt/Build"));
}

#[cfg(feature = "bundled")]
fn compile_jolt(build_path: &Path) -> PathBuf {
    let mut config = cmake::Config::new(build_path);
    config.profile("Release");
    config.build_target("ALL_BUILD");

    config.build()
}
