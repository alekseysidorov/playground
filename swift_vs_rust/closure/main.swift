func apply(f: (_: Int, _: Int) -> Int, _ v1: Int) -> (_: Int) -> Int
{
    return {(c: Int) -> Int in
        return f(v1, c)
    }
}

func make_sum(a: Int, second b: Int) -> Int
{
    return a + b
}

let a = 2; let b = 5;
let c = make_sum(a, second:b)
print("Swift: c is \(c)")

let f2 = apply(make_sum, b)
f2(a)

var d = 0;
for i in 0...1000000000 {
    d = f2(i);
}

print("Swift: d is \(d)");