#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Number(f64),
}

pub fn evaluate_addition(add: &Expression) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(next))
    } else {
        panic!("Addition not provided")
    }
}

pub fn evaluate_multiplication(mult: &Expression) -> f64 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(next))
    } else {
        panic!("Multiply not provided")
    }
}

pub fn evaluate_subtraction(sub: &Expression) -> f64 {
    if let Expression::Subtract(expressions) = sub {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first), |total, next| total - evaluate(next))
    } else {
        panic!("Subtract not provided")
    }
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression),
        Expression::Multiply(_) => evaluate_multiplication(expression),
        Expression::Subtract(_) => evaluate_subtraction(expression),
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

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    #[test]
    fn basic_addition() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];
        // act
        let sum = evaluate_addition(&Expression::Add(values));
        // assert
        assert_eq!(4.0, sum);
    }
    #[test]
    fn basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(4.0), Expression::Number(2.0)];
        // act
        let remainder = evaluate_subtraction(&Expression::Subtract(values));
        // assert
        assert_eq!(2.0, remainder);
    }
    #[test]
    fn basic_multiplication() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];
        // act
        let product = evaluate_multiplication(&Expression::Multiply(values));
        // assert
        assert_eq!(4.0, product);
    }
}
