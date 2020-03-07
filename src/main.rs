use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Variable(String),
    Number(f64),
}

pub struct Environment {
    hash: HashMap<String, Expression>
}

impl Environment {
    pub fn define(self: &mut Environment, key: String, value: Expression) {
        self.hash.insert(key, value);
    }

    pub fn resolve(self: &Environment, expression: &Expression) -> &Expression {
        println!("resolving {:?}", expression);
        if let Expression::Variable(name) = expression {
            if let Some(value) = self.hash.get(name) {
                value
            } else {
                panic!("variable not found");
            }
        } else {
            panic!("Trying to resolve an expression({:?}) that is not a variable", expression);
        }
    }
    pub fn new() -> Environment {
        Environment{hash: HashMap::new()}
    }
}

pub fn evaluate_addition(environment: &Environment, add: &Expression) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(environment, next))
    } else {
        panic!("Addition not provided")
    }
}

pub fn evaluate_multiplication(environment: &Environment, mult: &Expression) -> f64 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(environment, next))
    } else {
        panic!("Multiply not provided")
    }
}

pub fn evaluate_subtraction(environment: &Environment, sub: &Expression) -> f64 {
    if let Expression::Subtract(expressions) = sub {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(environment, first),
                |total, next| total - evaluate(environment, next))
    } else {
        panic!("Subtract not provided")
    }
}

pub fn evaluate_variable<'a>(environment: &'a Environment, var: &Expression) -> &'a Expression {
    if let Expression::Variable(_) = var {
        environment.resolve(var)
    } else {
        panic!("Variable not provided ({:?})", var)
    }
}

fn evaluate(environment: &Environment, expression: &Expression) -> f64 {
    match expression {
        Expression::Add(_) => evaluate_addition(environment, expression),
        Expression::Multiply(_) => evaluate_multiplication(environment, expression),
        Expression::Subtract(_) => evaluate_subtraction(environment, expression),
        Expression::Variable(_) => evaluate(environment,
            &evaluate_variable(environment, &expression)),
        Expression::Number(val) => *val,
    }
}


fn main() {
    // let mut expressions = Vec::new();
    // expressions.push(Expression::Number(3.0));
    // expressions.push(Expression::Number(4.0));
    // expressions.push(Expression::Number(5.0));
    // let add = Expression::Add(expressions);
    // let add2 = Expression::Multiply(vec![Expression::Number(2.5), add]);
    // let result = evaluate(&add2);
    // println!("result was {}", result);
    // let sub = Expression::Subtract(vec![add2, Expression::Number(2.2)]);
    // println!("result was {}", evaluate(&sub));
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
        let sum = evaluate_addition(&Environment::new(), &Expression::Add(values));
        // assert
        assert_eq!(4.0, sum);
    }
    #[test]
    fn basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(4.0), Expression::Number(2.0)];
        // act
        let remainder = evaluate_subtraction(&Environment::new(), &Expression::Subtract(values));
        // assert
        assert_eq!(2.0, remainder);
    }
    #[test]
    fn basic_multiplication() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];
        // act
        let product = evaluate_multiplication(&Environment::new(), &Expression::Multiply(values));
        // assert
        assert_eq!(4.0, product);
    }

    #[test]
    fn basic_environment_variable() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Number(2.0)];
        let addition = Expression::Add(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(7.0, sum);
    }
    #[test]
    fn two_environment_variables() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        environment.define(String::from("y"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Variable(String::from("y"))];
        let addition = Expression::Add(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(10.0, sum);
    }
    #[test]
    fn two_environment_variables_mult_impl_on_expression() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        environment.define(String::from("y"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Variable(String::from("y"))];
        let addition = Expression::Multiply(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(25.0, sum);
    }
}
