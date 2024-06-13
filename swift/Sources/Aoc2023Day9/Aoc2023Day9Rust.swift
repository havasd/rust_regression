import SwiftRs

@_cdecl("day9p1ForRust")
public func day9p1ForRust(_ input: SRString) -> Int64 {
  let result = day9p1(input: input.toString())
  return result
}

@_cdecl("day9p2ForRust")
public func day9p2ForRust(_ input: SRString) -> Int64 {
  let result = day9p2(input: input.toString())
  return result
}
