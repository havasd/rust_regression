import Foundation
import Aoc2023Day9

let file = "../input.txt"
let input = try String(contentsOfFile: file, encoding: .utf8)
var result = day9p1(input: input)
print("Result part1: \(result)")
result = day9p2(input: input)
print("Result part2: \(result)")
