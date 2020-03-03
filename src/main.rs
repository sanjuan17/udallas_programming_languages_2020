#[derive(Clone)]
#[derive(Debug)]
enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Number(f64),
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Add(expressions) => {
            let iter = expressions.iter();
            iter.fold(0.0, |total, next| total + evaluate(next))
        },
        Expression::Multiply(expressions) => {
            let iter = expressions.iter();
            iter.fold(1.0, |total, next| total * evaluate(next))
        },
        Expression::Subtract(expressions) => {
            let mut iter = expressions.iter();
            let first = iter.next().unwrap();
            iter.fold(evaluate(first), |total, next| total - evaluate(next))
        },
        Expression::Number(val) => *val,
    }
}

fn main() {
    let mut expressions = Vec::new();
    expressions.push(Expression::Number(3.0));
    expressions.push(Expression::Number(4.0));
    expressions.push(Expression::Number(5.0));
    let add = Expression::Add(expressions);
    let add2 = Expression::Multiply(vec![Expression::Number(2.5), add]);
    let result = evaluate(&add2);
    println!("result was {}", result);
    let sub = Expression::Subtract(vec![add2, Expression::Number(2.2)]);
    println!("result was {}", evaluate(&sub));
}
