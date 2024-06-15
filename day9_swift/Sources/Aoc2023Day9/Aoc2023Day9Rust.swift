import Foundation
import SwiftRs

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

@_cdecl("day9p1_rust_bridge2")
public func day9p1RustBridge2(_ input: UnsafeMutableRawPointer, _ len: Int ) -> Int64 {
  let d = Data(bytesNoCopy: input, count: len , deallocator: Data.Deallocator.none)
  let input = String(data: d, encoding: .utf8)!
  let result = day9p1(input: input)
  return result
}
