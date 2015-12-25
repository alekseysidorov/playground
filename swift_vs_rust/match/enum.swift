//
// enum.swift
// Swift VS Rust
//
// Created by <Name> on <Date>.
// Copyright <Year and company>. All rights reserved.
//

import Swift

public enum MathOperation {
  case Value(Int32)
  indirect case Sum(MathOperation, MathOperation)
  indirect case Mul(MathOperation, MathOperation)

  public func solve() -> Int32 {
    switch self {
    case .Value(let value):      return value
    case .Sum(let lhs, let rhs): return lhs.solve() + rhs.solve()
    case .Mul(let lhs, let rhs): return lhs.solve() * rhs.solve()
    }
  }
}

public protocol HasName {
  func name() -> String
}

extension MathOperation: HasName {
  public func name() -> String {
    switch self {
    case .Value(_):  return "Value"
    case .Sum(_, _): return "Sum"
    case .Mul(_, _): return "Mul"
    }
  }
}

let op = MathOperation.Sum(.Value(10), .Mul(.Value(20), .Value(2)))

print("Swift: op is \(op.name()) solved \(op.solve())")
