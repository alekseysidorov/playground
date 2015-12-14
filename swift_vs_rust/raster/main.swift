struct GenericPixmap<T> {
    var w: Int
    var h: Int

    var data: [T]

    init(width: Int, height: Int, fillValue: T) {
        self.w = width
        self.h = height
        self.data = [T](count: w*h, repeatedValue: fillValue)
    }

    func indexIsValid(x: Int, _ y: Int) -> Bool {
        return x >= 0 && x < w && y >= 0 && y < h
    }

    subscript(x: Int, y: Int) -> T {
        get {
            precondition(indexIsValid(x, y), "Index out-of-bounds")
            return data[x * y + y]
        }
        set {
            precondition(indexIsValid(x,y), "Index out-of-bounds")
            data[x * y + y] = newValue
        }
    }
}

typealias Pixmap = GenericPixmap<UInt32>

protocol Canvas {
    func setPixel(x: Int, _ y: Int, color: UInt32);
}

class MyCanvas : Canvas {
    var pixmap: Pixmap
    
    init(width: Int, height: Int, fillValue: UInt32) {
        self.pixmap = Pixmap(width:width, height:height, fillValue:fillValue)
    }
    
    func setPixel(x: Int, _ y: Int, color: UInt32)
    {
        pixmap[x, y] = color
    }
}

struct Vector3 {
    var x: Int;
    var y: Int;
    var z: Int;

    subscript(i: Int) -> Int {
        get {
            precondition(i >= 0 && i < 3, "Index out-of-bounds")
            switch i {
            case 0: return self.x
            case 1: return self.y
            case 2: return self.z
            default: return 0
            }
        }
        set {
            precondition(i >= 0 && i < 3, "Index out-of-bounds")
            switch i {
            case 0: self.x = newValue
            case 1: self.y = newValue
            case 2: self.z = newValue
            default: break
            }
        }        
    }
}

func == (left: Vector3, right: Vector3) -> Bool {
    return (left.x == right.x) && (left.y == right.y) && (left.z == right.z)
}
func != (left: Vector3, right: Vector3) -> Bool {
    return !(left == right)
}

class LineRaster {

    class State {
        var step: Vector3
        var d: Vector3
        var majorAxis: Int

        init() {
            self.step = Vector3(x: 0, y: 0, z: 0)
            self.d = Vector3(x: 0, y: 0, z: 0)
            self.majorAxis = 0
        }
    }

    var from: Vector3
    let to: Vector3
    var state: State?

    init(from: Vector3, to: Vector3) {
        self.from = from
        self.to = to
    }

    func next_point() -> Vector3? {
        if let state = self.state {
            if (self.from == self.to) {
                return nil
            } else {
                let calsResidualSteps = {axis in return abs(self.to[axis] - self.from[axis])}
                
                self.from[state.majorAxis] += state.step[state.majorAxis];
                let rsBase = calsResidualSteps(state.majorAxis);
                for i in 0..<3 {
                    let rs = calsResidualSteps(i);
                    
                    if rs > 0 && i != state.majorAxis {
                        state.d[i] += rs;
                        if state.d[i] >= rsBase {
                            state.d[i] -= rsBase;
                            self.from[i] += state.step[i];
                        }
                    }
                }
                
                return self.from
            }
        } else {
            let state = State()
            var max = 0;
            for i in 0..<3 {
                let d = self.to[i] - self.from[i];
                state.step[i] = d > 0 ? 1 : -1;
                
                let da = abs(d);
                if da > max {
                    max = da;
                    state.majorAxis = i;
                };                
            }
            self.state = state
            return self.from
        }
    }
}

extension LineRaster : GeneratorType {
    func next() -> Vector3? {
        return self.next_point()
    }
}

extension LineRaster : SequenceType {
    func generate() -> LineRaster {
        return self
    }
}

func testCode(canvas: Canvas) -> () {
    let a = Vector3(x: 0, y:0, z:0)
    let b = Vector3(x: 50, y:55, z:-20)
    let raster = LineRaster(from: a, to: b)
    for point in raster {
        let color = UInt32.max
        canvas.setPixel(point.x, point.y, color: color)
    }
}

var canvas = MyCanvas(width: 300, height: 300, fillValue: 0)

var a = Vector3(x: 0, y:0, z:0)
var b = Vector3(x: 10, y:5, z:-4)
let raster = LineRaster(from: a, to: b)
for point in raster {
    let color = UInt32.max
    canvas.setPixel(point.x, point.y, color: color)
    print("Swift: point: x: \(point.x), y: \(point.y), z:\(point.z), color: #\(color)")
}


for _ in 0..<100000 {
    testCode(canvas)
}