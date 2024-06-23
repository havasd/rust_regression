// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "Aoc2023Day9",
  platforms: [
    .macOS(.v10_13)
  ],
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "Aoc2023Day9",
      type: .static,
      targets: ["Aoc2023Day9"]),
    .executable(
      name: "Aoc2023Day9Executable",
      targets: ["Aoc2023Day9Executable"])
  ],

  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .target(
      name: "Aoc2023Day9"),
    .executableTarget(
      name: "Aoc2023Day9Executable",
      dependencies: ["Aoc2023Day9"]),
    .testTarget(
      name: "Aoc2023Day9Tests",
      dependencies: [
        "Aoc2023Day9"
      ])
  ]
)
