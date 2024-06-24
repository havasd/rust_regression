import Foundation
#if os(macOS)
import SwiftRs

/// these are bridging functions to allow Rust to call Swift functions

@_cdecl("day9p1_rust_bridge")
public func day9p1RustBridge(_ input: SRString) -> Int64 {
  let result = day9p1(input: input.toString())
  return result
}

@_cdecl("day9p2_rust_bridge")
public func day9p2RustBridge(_ input: SRString) -> Int64 {
  let result = day9p2(input: input.toString())
  return result
}

#endif
