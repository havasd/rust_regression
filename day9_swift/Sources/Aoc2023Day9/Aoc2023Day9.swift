public func day9p1(input: String) -> Int64 {
  var result: Int64 = 0
  for line in input.split(separator: "\n") {
    var sequence = line.split(separator: " ").map { Int64($0)! }
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

public func day9p2(input: String) -> Int64 {
  var result: Int64 = 0
  for line in input.split(separator: "\n") {
    var sequence = line.split(separator: " ").map { Int64($0)! }
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

