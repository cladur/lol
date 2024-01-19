use std::io::Write;

use parser::{parse, tokenize, visualize, Expression, Token};

mod parser;

/// Partially evaluate the L_int language.
fn partially_evaluate(exp: Expression) -> Expression {
    match exp {
        Expression::Number(n) => Expression::Number(n),
        Expression::Call(id, args) => match id.as_str() {
            "+" => {
                let mut sum = 0;
                let mut new_args = Vec::new();
                for arg in args.iter() {
                    match partially_evaluate(arg.clone()) {
                        Expression::Number(n) => sum += n,
                        Expression::Call(id, args) => {
                            new_args.push(Expression::Call(id, args));
                        }
                    }
                }
                if new_args.is_empty() {
                    Expression::Number(sum)
                } else {
                    new_args.insert(0, Expression::Number(sum));
                    Expression::Call("+".to_string(), new_args)
                }
            }
            "-" => {
                let mut new_args = Vec::new();
                let minuend = partially_evaluate(args[0].clone());
                let subtrahends = &args[1..];
                let mut subtrahend_sum = 0;
                for subtrahend in subtrahends.iter() {
                    match partially_evaluate(subtrahend.clone()) {
                        Expression::Number(n) => subtrahend_sum += n,
                        Expression::Call(id, args) => {
                            new_args.push(Expression::Call(id, args));
                        }
                    }
                }
                match minuend {
                    Expression::Number(n) => {
                        if new_args.is_empty() {
                            Expression::Number(n - subtrahend_sum)
                        } else {
                            new_args.insert(0, Expression::Number(n - subtrahend_sum));
                            Expression::Call("-".to_string(), new_args)
                        }
                    }
                    Expression::Call(id, args) => {
                        new_args.insert(0, Expression::Call(id, args));
                        new_args.push(Expression::Number(subtrahend_sum));
                        Expression::Call("-".to_string(), new_args)
                    }
                }
            }
            "read" => {
                // If we have a read, we can't partially evaluate it
                Expression::Call(id, args)
            }
            _ => {
                // If we have an unknown function, we can't partially evaluate it
                Expression::Call(id, args)
            }
        },
    }
}

/// Interpret the L_int language.
fn interpret(exp: Expression) -> i32 {
    match exp {
        Expression::Number(n) => n,
        Expression::Call(id, args) => match id.as_str() {
            "+" => {
                let mut sum = 0;
                for arg in args {
                    sum += interpret(arg);
                }
                sum
            }
            "-" => {
                let mut sum = interpret(args[0].clone());
                for arg in args[1..].iter() {
                    sum -= interpret(arg.clone());
                }
                sum
            }
            "read" => {
                let mut input = String::new();
                print!("> ");
                // flush
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                input.trim().parse().unwrap()
            }
            _ => {
                0 // Placeholder value, replace with appropriate code
            }
        },
    }
}

fn main() {
    let input = "(+ 20 1 (read) 15 (- 10 3))";
    let tokens = tokenize(input);
    let ast = parse(&tokens);

    println!("AST: {}", visualize(&ast));

    let pe_ast = partially_evaluate(ast.clone());

    println!("Partially evaluated AST: {}", visualize(&pe_ast));

    println!("{}", interpret(ast));
    println!("{}", interpret(pe_ast));
}
