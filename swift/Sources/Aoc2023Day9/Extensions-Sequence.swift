// This should be in the standard library, generic for any collection type.
extension Sequence {
  func anySatisfy(_ predicate: (Element) -> Bool) -> Bool {
    for element in self {
      if predicate(element) {
        return true
      }
    }
    return false
  }

  func scan<State, Result>(
    _ initialResult: State, _ nextPartialResult: (inout State, Element) -> Result
  )
    -> [Result]
  {
    var result: [Result] = []
    var acc = initialResult
    for element in self {
      result.append(nextPartialResult(&acc, element))
    }
    return result
  }
}
