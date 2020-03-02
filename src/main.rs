#[derive(Clone)]
#[derive(Debug)]
enum Primitive {
    Add(Vec<Primitive>),
    Multiply(Vec<Primitive>),
    Subtract(Vec<Primitive>),
    Number(i32),
}

fn evaluate(primitive: &Primitive) -> i32 {
    match primitive {
        Primitive::Add(primitives) => {
            let iter = primitives.iter();
            iter.fold(0, |total, next| total + evaluate(next))
        },
        Primitive::Multiply(primitives) => {
            let iter = primitives.iter();
            iter.fold(1, |total, next| total * evaluate(next))
        },
        Primitive::Subtract(primitives) => {
            let mut iter = primitives.iter();
            let first = iter.next().unwrap();
            iter.fold(evaluate(first), |total, next| total - evaluate(next))
        },
        Primitive::Number(val) => *val,
    }
}

fn main() {
    let mut primitives = Vec::new();
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(5));
    let add = Primitive::Add(primitives);
    let add2 = Primitive::Multiply(vec![Primitive::Number(2), add]);
    let result = evaluate(&add2);
    println!("result was {}", result);
    let sub = Primitive::Subtract(vec![Primitive::Number(4), Primitive::Number(2)]);
    println!("result was {}", evaluate(&sub));
}
