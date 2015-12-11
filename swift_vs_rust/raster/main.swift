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
            assert(indexIsValid(x, y))
            return data[x * y + y]
        }
        set {
            assert(indexIsValid(x,y))
            data[x * y + y] = newValue
        }
    }
}

typealias Pixmap = GenericPixmap<UInt32>

protocol Canvas {
    func setPixel(x: Int, y: Int, color: UInt32);
}

class MyCanvas : Canvas
{
    var pixmap: Pixmap
    
    init(width: Int, height: Int, fillValue: UInt32) {
        self.pixmap = Pixmap(width:width, height:height, fillValue:fillValue)
    }
    
    func setPixel(x: Int, y: Int, color: UInt32)
    {
        pixmap[x, y] = color
    }
}
