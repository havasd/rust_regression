import Foundation
#if os(macOS)

/// these are bridging functions to allow Rust to call Swift functions

@_cdecl("day9p1_rust_bridge")
public func day9p1RustBridge(_ input: UnsafeMutablePointer<UInt8>, _ len: UInt64) -> Int64 {
  let result = day9p1(input: Data(bytesNoCopy: input, count: Int(len) , deallocator: .none))
  return result
}

@_cdecl("day9p2_rust_bridge")
public func day9p2RustBridge(_ input: UnsafeMutablePointer<UInt8>, _ len: UInt64) -> Int64 {
  let result = day9p2(input: Data(bytesNoCopy: input, count: Int(len) , deallocator: .none))
  return result
}

#endif
