//
// main.swift
// Swift VS Rust
//
// Created by <Name> on <Date>.
// Copyright <Year and company>. All rights reserved.
//

import Swift

// MARK: Utils

internal class _Buffer<Element>: ManagedBuffer<Int,Element> {
  deinit {
    self.withUnsafeMutablePointerToElements { $0.destroy(self.value) }
  }
}

internal extension _Buffer {
  internal func clone() -> _Buffer<Element> {
    return self.withUnsafeMutablePointerToElements { clonedElements in
      return _Buffer.create(self.allocatedElementCount) { newBuffer in
        newBuffer.withUnsafeMutablePointerToElements { newElements in
          newElements.initializeFrom(clonedElements, count: self.value)
        }

        return self.value
      } as! _Buffer
    }
  }

  internal func resize(newSize: Int) -> _Buffer<Element> {
    return self.withUnsafeMutablePointerToElements { elements in
      return _Buffer.create(newSize) { newBuffer in
        defer {
          // Tell the old buffer that it no longer
          // has any elements to manage.
          self.value = 0
        }

        newBuffer.withUnsafeMutablePointerToElements { newElems in
          newElems.moveInitializeFrom(elements, count: self.value)
        }

        return self.value
      } as! _Buffer
    }
  }
}

// MARK: Implementation

public struct Pixmap<Pixel> {
  // Using UnsafeMutablePointer<Element> increases perfomace by 0.3 seconds,
  // but I (contributor) am to lazy to implement efficient deallocator for allocated memory.
  internal private(set) var buffer: _Buffer<Pixel>
  public internal(set) var width, height: Int

  public init(width: Int, height: Int, fillValue: Pixel) {
    self.width = width
    self.height = height
    self.buffer = _Buffer.create(width * height) { _ in 0 } as! _Buffer
  }
}

extension Pixmap: MutableCollectionType {
  public var startIndex: Int { return 0 }
  public var endIndex: Int { return self.buffer.value }

  /// Return element at `position` from the underlying storage.
  public subscript(position: Int) -> Pixel {
    get {
      return self.buffer.withUnsafeMutablePointerToElements { $0[position] }
    }

    mutating set {
      if !isUniquelyReferenced(&self.buffer) {
        self.buffer = buffer.clone()
      }

      self.buffer.withUnsafeMutablePointerToElements { $0[position] = newValue }
    }
  }
}

public extension Pixmap {
  /// Return row at `position` from the underlying storage.
  public subscript(row position: Int) -> Pixmap.SubSequence {
    get {
      let from = self.width * position
      let to   = self.width + from

      return self[from..<to]
    }
  }

  public subscript(x: Int, y: Int) -> Pixel {
    get {
      return self[row: x][y]
    }

    mutating set {
      let position = (self.width * x) + y
      self[position] = newValue
    }
  }
}

public protocol CanvasType {
  typealias Value
  subscript(x: Int, y: Int) -> Value { get mutating set }
  mutating func setPixel(x: Int, _ y: Int, color: Value)
}

extension CanvasType {
  public mutating func setPixel(x: Int, _ y: Int, color: Value) {
    self[x, y] = color
  }
}

extension Pixmap: CanvasType {}

typealias Canvas = Pixmap<UInt32>

public struct Vector3 {
  public var x, y, z: Int

  public init(x: Int = 0, y: Int = 0, z: Int = 0) {
    self.x = x
    self.y = y
    self.z = z
  }

  public subscript(position: Int) -> Int {
    get {
      switch position {
      case 0: return self.x
      case 1: return self.y
      case 2: return self.z
      default: fatalError("Index out-of-bounds")
      }
    }

    mutating set {
      switch position {
      case 0: self.x = newValue
      case 1: self.y = newValue
      case 2: self.z = newValue
      default: fatalError("Index out-of-bounds")
      }
    }
  }
}

public func ==(lhs: Vector3, rhs: Vector3) -> Bool {
  return (lhs.x == rhs.x) && (lhs.y == rhs.y) && (lhs.z == rhs.z)
}

public func !=(lhs: Vector3, rhs: Vector3) -> Bool {
  return !(lhs == rhs)
}

public struct LineRaster {
  public var from: Vector3
  public internal(set) var to: Vector3
  internal private(set) var state: Optional<LineRaster.State>

  internal struct State {
    internal var step, d: Vector3
    internal var majorAxis: Int

    internal init(step: Vector3 = Vector3(), d: Vector3 = Vector3(), majorAxis: Int = 0) {
      self.majorAxis = majorAxis
      self.step = step
      self.d = d
    }
  }

  public init(from: Vector3, to: Vector3) {
    self.from = from
    self.to = to
  }

  @inline(__always)
  public mutating func nextPoint() -> Optional<Vector3> {
    var state = self.state ?? LineRaster.State()
    defer {
      self.state = state
    }

    switch self.state {
    case .None:
      var max = 0
      for i in 0..<3 {
        var d = self.to[i] - self.from[i]
        state.step[i] = d > 0 ? 1 : -1

        d = abs(d)
        if d > max {
          max = d
          state.majorAxis = i
        }
      }

    case .Some(_):
      guard self.from != self.to else { return nil }

      @inline(__always)
      func calsResidualSteps(axis: Int) -> Int {
        return abs(self.to[axis] - self.from[axis])
      }

      self.from[state.majorAxis] += state.step[state.majorAxis]
      let base = calsResidualSteps(state.majorAxis)
      for i in 0..<3 where i != state.majorAxis {
        let rs = calsResidualSteps(i)

        if rs > 0 {
          state.d[i] += rs
          if state.d[i] >= base {
            state.d[i] -= base
            self.from[i] += state.step[i]
          }
        }
      }
    }

    return self.from
  }
}

extension LineRaster: SequenceType {
  public func generate() -> AnyGenerator<Vector3> {
    var this = self
    // Swift 2.1: `anyGenerator`
    // Swift 2.2: `AnyGenerator`
    return AnyGenerator {
      return this.nextPoint()
    }
  }
}

internal func testCode(canvas: Canvas) {
  var canvas = canvas

  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas[point.x, point.y] = color
  }
}

internal func testCodeInout(inout canvas: Canvas) {
  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas[point.x, point.y] = color
  }
}

internal func testCodeGeneric<T: CanvasType where T.Value == UInt32>(canvas: T) {
  var canvas = canvas

  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas[point.x, point.y] = color
  }
}

var canvas = Canvas(width: 300, height: 300, fillValue: 0)

let a = Vector3(x: 0,  y: 0, z:  0)
let b = Vector3(x: 10, y: 5, z: -4)
let raster = LineRaster(from: a, to: b)

for point in raster {
  let color = UInt32.max
  canvas[point.x, point.y] = color
  print("Swift: point: x: \(point.x), y: \(point.y), z:\(point.z), color: #\(String(color, radix: 16))")
}

// var myCanvas = canvas
// for _ in 0..<1000000 {
//   testCode(myCanvas)
// }
//
// for _ in 0..<1000000 {
//   testCodeGeneric(canvas)
// }

for _ in 0..<1000000 {
  testCodeInout(&canvas)
}
