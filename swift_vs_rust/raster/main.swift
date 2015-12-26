//
// main.swift
// Swift VS Rust
//
// Created by <Name> on <Date>.
// Copyright <Year and company>. All rights reserved.
//

import Swift

public struct Pixmap<Element> {
  // Using UnsafeMutablePointer<Element> increases perfomace by 0.3 seconds,
  // but I am to lazy to implement efficient deallocator for allocated memory.
  internal private(set) var data: Array<Element>
  public internal(set) var width, height: Int

  public init(width: Int, height: Int, fillValue: Element) {
    self.width = width
    self.height = height
    // AutoreleasingUnsafeMutablePointer(UnsafeMutablePointer.alloc(width * height))
    self.data = Array(count: width * height, repeatedValue: fillValue)
  }

  // internal func isIndexValid(x: Int, _ y: Int) -> Bool {
  //     return x >= 0 && x < w && y >= 0 && y < h
  // }

  public subscript(x: Int, y: Int) -> Element {
    get {
      // precondition(self.isIndexValid(x, y), "Index out-of-bounds")
      return data[x * y + y]
    }

    mutating set {
      // precondition(self.isIndexValid(x,y), "Index out-of-bounds")
      data[x * y + y] = newValue
    }
  }
}

public protocol CanvasType {
  mutating func setPixel(x: Int, _ y: Int, color: UInt32)
}

public struct Canvas: CanvasType {
  public internal(set) var pixmap: Pixmap<UInt32>

  public init(width: Int, height: Int, fillValue: UInt32) {
    self.pixmap = Pixmap<UInt32>(width: width, height: height, fillValue: fillValue)
  }

  public mutating func setPixel(x: Int, _ y: Int, color: UInt32) {
    pixmap[x, y] = color
  }
}

public struct Vector3 {
  public var x, y, z: Int

  public subscript(position: Int) -> Int {
    get {
      //precondition(i >= 0 && i < 3, "Index out-of-bounds")
      switch position {
      case 0: return self.x
      case 1: return self.y
      case 2: return self.z
      default: return 0
      }
    }

    mutating set {
      //precondition(i >= 0 && i < 3, "Index out-of-bounds")
      switch position {
      case 0: self.x = newValue
      case 1: self.y = newValue
      case 2: self.z = newValue
      default: break
      }
    }
  }
}

public func ==(left: Vector3, right: Vector3) -> Bool {
  return (left.x == right.x) && (left.y == right.y) && (left.z == right.z)
}

public func !=(left: Vector3, right: Vector3) -> Bool {
  return !(left == right)
}

public struct RasterState {
  public var step, d: Vector3
  public var majorAxis: Int

  public init() {
    self.step = Vector3(x: 0, y: 0, z: 0)
    self.d = Vector3(x: 0, y: 0, z: 0)
    self.majorAxis = 0
  }
}

public struct LineRaster {
  public var from: Vector3
  public internal(set) var to: Vector3
  public var state: RasterState?

  public init(from: Vector3, to: Vector3) {
    self.from = from
    self.to = to
  }

  public mutating func nextPoint() -> Vector3? {
    // WARNING: `if var _ = _ {}` and `guard var _ = _ else {}`
    // patterns will be deprecated in the future Swift versions.
    guard var state = self.state else {
      var state = RasterState()
      var max = 0
      for i in 0..<3 {
        let d = self.to[i] - self.from[i]
        state.step[i] = d > 0 ? 1 : -1

        let da = abs(d)
        if da > max {
          max = da
          state.majorAxis = i
        }
      }
      self.state = state
      return self.from
    }

    guard self.from != self.to else { return nil }

    let calsResidualSteps = { axis in abs(self.to[axis] - self.from[axis]) }

    self.from[state.majorAxis] += state.step[state.majorAxis]
    let rsBase = calsResidualSteps(state.majorAxis)
    for i in 0..<3 where i != state.majorAxis {
      let rs = calsResidualSteps(i)

      if rs > 0 {
        state.d[i] += rs
        if state.d[i] >= rsBase {
          state.d[i] -= rsBase
          self.from[i] += state.step[i]
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

internal func testCode(canvas: CanvasType) {
  var canvas = canvas

  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas.setPixel(point.x, point.y, color: color)
  }
}

internal func testCodeInout(inout canvas: CanvasType) {
  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas.setPixel(point.x, point.y, color: color)
  }
}

internal func testCodeGeneric<T: CanvasType>(canvas: T) {
  var canvas = canvas

  let a = Vector3(x: 0, y:0, z:0)
  let b = Vector3(x: 50, y:55, z:-20)
  let raster = LineRaster(from: a, to: b)
  for point in raster {
    let color = UInt32.max
    canvas.setPixel(point.x, point.y, color: color)
  }
}

var canvas: CanvasType = Canvas(width: 300, height: 300, fillValue: 0)

let a = Vector3(x: 0,  y: 0, z:  0)
let b = Vector3(x: 10, y: 5, z: -4)
let raster = LineRaster(from: a, to: b)

for point in raster {
  let color = UInt32.max
  canvas.setPixel(point.x, point.y, color: color)
  print("Swift: point: x: \(point.x), y: \(point.y), z:\(point.z), color: #\(String(color, radix: 16))")
}

// var myCanvas: CanvasType = canvas
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
