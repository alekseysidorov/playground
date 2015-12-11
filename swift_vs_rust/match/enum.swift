enum MathOperation {
  case Value(Int)
  indirect case Sum(MathOperation, MathOperation)
  indirect case Mul(MathOperation, MathOperation)
  
  func solve() -> Int {
    switch self {
    case .Value(let value):
        return value
    case .Sum(let left, let right):
        return left.solve() + right.solve()
    case .Mul(let left, let right):
        return left.solve() * right.solve()
    }
  }
}

protocol HasName {
  func name() -> String;
}

extension MathOperation : HasName
{
  func name() -> String {
    switch self {
      case .Value(_):
        return "Value"
      case .Sum(_, _):
        return "Sum"
      case .Mul(_, _):
        return "Mul"
      }
  }
}

let op = MathOperation.Sum(MathOperation.Value(10),
                           MathOperation.Mul(MathOperation.Value(20),
                                             MathOperation.Value(2)))
                                             
print("Swift: op is \(op.name()) solved \(op.solve())");

