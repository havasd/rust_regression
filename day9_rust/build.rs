// build.rs

use std::path::PathBuf;

#[cfg(target_os="macos")]
fn main() {
    let out_dir = PathBuf::from("./generated");

    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}

#[cfg(not(target_os="macos"))]
fn main() {}
