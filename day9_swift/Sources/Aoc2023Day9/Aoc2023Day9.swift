import Foundation   
public func day9p1(input: Data) -> Int64 {
  var result: Int64 = 0
  for line in input.split(separator: UInt8(ascii: "\n")) {
    var sequence = line.split(separator: UInt8(ascii: " ")).map { Int64(String(data: $0, encoding: .utf8)!)! }
    while sequence.first(where: { $0 != 0 }) != nil {
      result += sequence.last!
      for i in 1..<sequence.count {
        sequence[i - 1] = sequence[i] - sequence[i - 1]
      }
      sequence.removeLast()
    }
  }
  return result
}

public func day9p2(input: Data) -> Int64 {
  var result: Int64 = 0
  for line in input.split(separator: UInt8(ascii: "\n")) {
    var sequence = line.split(separator: UInt8(ascii: " ")).map { Int64(String(data: $0, encoding: .utf8)!)! }
    sequence.reverse()
    while sequence.first(where: { $0 != 0 }) != nil {
      result += sequence.last!
      for i in 1..<sequence.count {
        sequence[i - 1] = sequence[i] - sequence[i - 1]
      }
      sequence.removeLast()
    }
  }
  return result
}

