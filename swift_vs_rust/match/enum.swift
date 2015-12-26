//
// enum.swift
// Swift VS Rust
//
// Created by <Name> on <Date>.
// Copyright <Year and company>. All rights reserved.
//

import Swift

public enum ArithmeticOperation<ArithmeticValue: IntegerArithmeticType> {
  case Value(ArithmeticValue)
  indirect case Sum(ArithmeticOperation, ArithmeticOperation)
  indirect case Mul(ArithmeticOperation, ArithmeticOperation)

  public func solve() -> ArithmeticValue {
    switch self {
    case .Value(let value):      return value
    case .Sum(let lhs, let rhs): return lhs.solve() + rhs.solve()
    case .Mul(let lhs, let rhs): return lhs.solve() * rhs.solve()
    }
  }
}

public protocol Nameable {
  var name: String { get }
}

extension ArithmeticOperation: Nameable {
  public var name: String {
    switch self {
    case .Value(_):  return "Value"
    case .Sum(_, _): return "Sum"
    case .Mul(_, _): return "Mul"
    }
  }
}

let op = ArithmeticOperation.Sum(.Value(10), .Mul(.Value(20), .Value(2)))

print("Swift: op is \(op.name) solved \(op.solve())")
