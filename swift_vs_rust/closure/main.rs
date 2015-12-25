fn apply<F: 'static>(f: F, v1: i32) -> Box<Fn(i32) -> i32>
    where F: Fn(i32, i32) -> i32
{
    Box::new(move |v2| f(v1, v2))
}

fn make_sum(x: i32, y: i32) -> i32
{
    x + y
}

fn main() {
    let a = 2; let b = 5;
    let c = make_sum(a, b);
    println!("Rust:  c is {}", c);

    let f2 = apply(make_sum, b);

    let mut tmp = 0;
    for i in 0..1000000000 {
        d = f2(i);
    }

    println!("Rust:  tmp is {}", tmp);
}
