trait HasName {
    fn name(&self) -> &'static str;
}

enum MathOperation {
    Value(i32),
    Sum(Box<MathOperation>, Box<MathOperation>),
    Mul(Box<MathOperation>, Box<MathOperation>)
}

impl HasName for MathOperation {
    fn name(&self) -> &'static str {
        match *self {
            MathOperation::Value(_) => "Value",
            MathOperation::Sum(_,_) => "Sum",
            MathOperation::Mul(_,_) => "Mul"
        }
    }
}

impl MathOperation {
    fn solve(&self) -> i32 {
        match *self {
            MathOperation::Value(i)         => i,
            MathOperation::Sum(ref left, ref right) => left.solve() + right.solve(),
            MathOperation::Mul(ref left, ref right) => left.solve() * right.solve()
        }
    }
}

fn main() {
    let op = MathOperation::Sum(Box::new(MathOperation::Value(10)),
                                Box::new(MathOperation::Mul(Box::new(MathOperation::Value(20)),
                                                            Box::new(MathOperation::Value(2)))));
                                
    ;
    println!("Rust: op is {} solved {}", op.name(), op.solve());
}
