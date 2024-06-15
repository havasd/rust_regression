#[cfg(target_os = "macos")]
use swift_rs::SwiftLinker;

#[cfg(target_os = "macos")]
fn main() {
    // swift-rs has a minimum of macOS 10.13
    // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    SwiftLinker::new("10.13")
        .with_package("Aoc2023Day9", "day9_swift")
        .link();

    // Other build steps
}

#[cfg(not(target_os="macos"))]
fn main() {}
