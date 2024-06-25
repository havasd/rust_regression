import XCTest

@testable import Aoc2023Day9
#if os(macOS)
@testable import Day9Rust
#endif

final class Aoc2023Day9Tests: XCTestCase {



  func testDay9p1Measure() {
    let file = "../input.txt"
    let input = try? Data(
      contentsOf: URL(filePath: file))

    #if os(macOS)
      let options = XCTMeasureOptions()
      options.iterationCount = 100
      measure(
        metrics: [XCTCPUMetric(), XCTMemoryMetric()],
        options: options
      ) {
        let result = day9p1(input: input!)
        XCTAssertEqual(result, 2_043_677_056)
      }
    #else
      measure {
        let result = day9p1(input: input!)
        XCTAssertEqual(result, 2_043_677_056)
      }
    #endif
  }

  func testDay9p2Measure() {
    let file = "../input.txt"
    let input = try? String(
      contentsOfFile: file,
      encoding: .utf8)
    #if os(macOS)
      let options = XCTMeasureOptions()
      options.iterationCount = 100
      measure(
        metrics: [XCTCPUMetric(), XCTMemoryMetric()],
        options: options
      ) {
        let result = day9p2(input: input!)
        XCTAssertEqual(result, 1062)
      }
    #else
      measure {
        let result = day9p2(input: input!)
        XCTAssertEqual(result, 1062)
      }
    #endif
  }

#if os(macOS)
    func testDay9p1RustMeasure() {
    let file = "../input.txt"
    let input = try? String(
      contentsOfFile: file,
      encoding: .utf8)
      let options = XCTMeasureOptions()
      options.iterationCount = 100
      measure(
        metrics: [XCTCPUMetric(), XCTMemoryMetric()],
        options: options
      ) {
        let result = Day9Rust.p1(RustString(input!).as_str())
        XCTAssertEqual(result, 2_043_677_056)
      }
  }

  func testDay9p2RustMeasure() {
    let file = "../input.txt"
    let input = try? String(
      contentsOfFile: file,
      encoding: .utf8)
      let options = XCTMeasureOptions()
      options.iterationCount = 100
      measure(
        metrics: [XCTCPUMetric(), XCTMemoryMetric()],
        options: options
      ) {
        let result = Day9Rust.p2(RustString(input!).as_str())
        XCTAssertEqual(result, 1062)
      }
  }
#endif

  static var allTests = [
    (
      "testDay9Swift",  testDay9p2Sample,
      testDay9p1Measure, testDay9p2Measure
    )
  ]
}
