import Foundation
import SwiftRs

public func day9p1(input: String) -> Int64 {
  let result = input.split(separator: "\n")
    .map(parseLineIntoSequenceOfNumbers)
    .map(predictNextValue)
    .reduce(0) { (result, value) in
      result + value
    }

  return result
}

@_cdecl("day9p1ForRust")
public func day9p1ForRust(_ input: SRString) -> Int64 {
  let result = day9p1(input: input.toString())
  return result
}

public func day9p2(input: String) -> Int64 {
  let result  = input.split(separator: "\n")
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
@_cdecl("day9p2ForRust")
public func day9p2ForRust(_ input: SRString) -> Int64 {
  let result = day9p2(input: input.toString())
  return result
}
func parseLineIntoSequenceOfNumbers(line: Substring) -> [Int64] {
  return line.split(separator: " ").map { Int64($0)! }
}

// This should be in the standard library, generic for any collection type.
extension [Int64] {
  func anySatisfy(_ predicate: (Int64) -> Bool) -> Bool {
    for element in self {
      if predicate(element) {
        return true
      }
    }
    return false
  }
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
  var result = [Int64]()

  for i in 0..<sequence.count - 1 {
    result.append(sequence[i + 1] - sequence[i])
  }

  return result
}
