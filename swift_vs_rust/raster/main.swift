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
    func setPixel(x: Int, y: Int, color: UInt32);
}

class MyCanvas : Canvas {
    var pixmap: Pixmap
    
    init(width: Int, height: Int, fillValue: UInt32) {
        self.pixmap = Pixmap(width:width, height:height, fillValue:fillValue)
    }
    
    func setPixel(x: Int, y: Int, color: UInt32)
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
            precondition(i < 0 || i >= 3, "Index out-of-bounds")
            switch i {
            case 0: return self.x
            case 1: return self.y
            case 2: return self.z
            default: return 0
            }
        }
        set {
            precondition(i < 0 || i >= 3, "Index out-of-bounds")            
            switch i {
            case 0: self.x = newValue
            case 1: self.y = newValue
            case 2: self.z = newValue
            default: break
            }
        }        
    }
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
            //if (self.from == self.to) {
                return nil
            //} else {

            //}
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