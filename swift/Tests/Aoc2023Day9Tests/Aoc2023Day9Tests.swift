import XCTest

@testable import Aoc2023Day9

final class Aoc2023Day9Tests: XCTestCase {
  func testDay9p1Sample() {
    let input = """
      0 3 6 9 12 15
      1 3 6 10 15 21
      10 13 16 21 30 45
      """
    let result = day9p1(input: input)
    XCTAssertEqual(result, 114)

  }

  func testDay9p2Sample() {
    let input = """
      0 3 6 9 12 15
      1 3 6 10 15 21
      10 13 16 21 30 45
      """
    let result = day9p2(input: input)
    XCTAssertEqual(result, 2)
  }
  
  static var allTests = [
    ("testDay9p1Sample", testDay9p1Sample, testDay9p2Sample)
  ]
}
