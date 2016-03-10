//
// main.swift
// Swift VS Rust
//
// Created by <Name> on <Date>.
// Copyright <Year and company>. All rights reserved.
//

/// Curry function `ƒ` with `toValue` as first parameter.
public func apply(toValue: Int, _ ƒ: (Int, Int) -> Int) -> Int -> Int {
    return { return ƒ(toValue, $0) }
}

public func sum(x: Int, _ y: Int) -> Int {
    return x + y
}

let a = 2, b = 5
let c = sum(a, b)
print("Swift: c is \(c)")

let f = apply(b, sum) // apply(b) { $0 + $1 }
f(a)

var tmp = 0
for element in 0..<1000000000 {
    tmp = f(element)
}

print("Swift: tmp is \(tmp)");
