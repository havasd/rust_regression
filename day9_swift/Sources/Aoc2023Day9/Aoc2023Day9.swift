public func day9p1(input: String) -> Int64 {
  let result = input.split(separator: "\n")
    .map(parseLineIntoSequenceOfNumbers)
    .map(predictNextValue)
    .reduce(0) { (result, value) in
      result + value
    }

  return result
}

public func day9p2(input: String) -> Int64 {
  let result = input.split(separator: "\n")
    .map(parseLineIntoSequenceOfNumbers)
    .map {
      $0.reversed()
    }
    .map(predictNextValue)
    .reduce(0) { (result, value) in
      result + value
    }

  return result
}

func parseLineIntoSequenceOfNumbers(line: Substring) -> [Int64] {
  return line.split(separator: " ").map { Int64($0)! }
}

func predictNextValue(sequence: [Int64]) -> Int64 {
  var result: Int64 = 0
  // This makes sequence mutable, but as we never write to it
  // but just replace it, copy-on-write should not be a problem.
  var sequence = sequence

  while sequence.anySatisfy({ $0 != 0 }) {
    // As anySatisfy returns false for an empty sequence,
    // the bang is safe here.
    result += sequence.last!
    sequence = generateDifferences(sequence: sequence)
  }
  return result
}

func generateDifferences(sequence: [Int64]) -> [Int64] {
  let firstvalue = sequence.first!
  let result = sequence[1..<sequence.count].scan(firstvalue) { (previous, current) in
    defer {
      previous = current
    }
    return current - previous
  }
  return result
}
