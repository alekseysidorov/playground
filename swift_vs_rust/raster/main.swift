struct GenericPixmap<T> {
    var w: Int
    var h: Int

    var data: [T]

    init(width: Int, height: Int, fillValue: T) {
        self.w = width
        self.h = height
        self.data = [T](count: w*h, repeatedValue: fillValue)
    }
}
