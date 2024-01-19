use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    LParen,
    RParen,
    EndOfFile,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LInt {
    Number(i32),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Read(),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LVar {
    Let(Vec<Binding>, Box<Expression>),
    Var(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    LInt(LInt),
    LVar(LVar),
}

pub type Env = std::collections::HashMap<String, i32>;

/// Takes a string and returns a vector of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            number.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' => {
                            identifier.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens.push(Token::EndOfFile);

    tokens
}

/// Takes a vector of tokens and returns an AST.
pub fn parse(tokens: &[Token]) -> Expression {
    let mut tokens = tokens.iter().peekable();
    parse_expression(&mut tokens)
}

fn parse_expression(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Expression {
    match tokens.peek() {
        Some(&Token::Number(_)) => parse_number(tokens),
        Some(&Token::Identifier(_)) => parse_identifier(tokens),
        Some(&Token::LParen) => {
            assert!(tokens.next() == Some(&&Token::LParen));

            let tok = tokens.next().unwrap();
            let mut args = Vec::new();

            if *tok == Token::Identifier("let".to_string()) {
                let mut bindings = Vec::new();

                // Skip LParen
                assert!(tokens.next() == Some(&&Token::LParen));

                // Parse bindings
                while tokens.peek() != Some(&&Token::RParen) {
                    // Skip LParen
                    assert!(tokens.next() == Some(&&Token::LParen));

                    let name = match tokens.next() {
                        Some(Token::Identifier(id)) => id,
                        _ => panic!("Expected identifier"),
                    };
                    let value = parse_expression(tokens);

                    // Skip RParen
                    assert!(tokens.next() == Some(&&Token::RParen));

                    bindings.push(Binding {
                        name: name.to_string(),
                        value,
                    });
                }

                // Skip RParen
                assert!(tokens.next() == Some(&&Token::RParen));

                return Expression::LVar(LVar::Let(bindings, Box::new(parse_expression(tokens))));
            }

            while tokens.peek() != Some(&&Token::RParen) {
                args.push(parse_expression(tokens));
            }
            assert!(tokens.next() == Some(&&Token::RParen));
            match tok {
                Token::Identifier(id) => match id.as_str() {
                    "+" => Expression::LInt(LInt::Add(
                        Box::new(args[0].clone()),
                        Box::new(args[1].clone()),
                    )),
                    "-" => Expression::LInt(LInt::Subtract(
                        Box::new(args[0].clone()),
                        Box::new(args[1].clone()),
                    )),
                    "read" => Expression::LInt(LInt::Read()),
                    _ => panic!("Unexpected token"),
                },
                _ => panic!("Unexpected token"),
            }
        }
        Some(&Token::RParen) => panic!("Unexpected RParen"),
        _ => panic!("Unexpected token"),
    }
}

fn parse_number(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Expression {
    match tokens.next() {
        Some(Token::Number(n)) => Expression::LInt(LInt::Number(*n)),
        _ => panic!("Expected number"),
    }
}

fn parse_identifier(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Expression {
    match tokens.next() {
        Some(Token::Identifier(id)) => Expression::LVar(LVar::Var(id.to_string())),
        _ => panic!("Expected identifier"),
    }
}

pub trait Eval {
    fn eval(&self, env: &mut Env) -> i32;
}

impl Eval for LInt {
    fn eval(&self, env: &mut Env) -> i32 {
        match self {
            LInt::Number(n) => *n,
            LInt::Add(a, b) => a.eval(env) + b.eval(env),
            LInt::Subtract(a, b) => a.eval(env) - b.eval(env),
            LInt::Read() => {
                let mut input = String::new();
                print!("> ");
                // flush
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                input.trim().parse().unwrap()
            }
        }
    }
}

impl Eval for LVar {
    fn eval(&self, env: &mut Env) -> i32 {
        match self {
            LVar::Let(bindings, body) => {
                for binding in bindings {
                    let value = binding.value.eval(env);
                    env.insert(binding.name.clone(), value);
                }
                body.eval(env)
            }
            LVar::Var(name) => env.get(name).unwrap().clone(),
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> i32 {
        let mut env = Env::new();
        self.eval(&mut env)
    }
}

impl Eval for Expression {
    fn eval(&self, env: &mut Env) -> i32 {
        match self {
            Expression::LInt(lint) => lint.eval(env),
            Expression::LVar(lvar) => lvar.eval(env),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::LInt(lint) => write!(f, "{}", lint),
            Expression::LVar(lvar) => write!(f, "{}", lvar),
        }
    }
}

impl std::fmt::Display for LInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LInt::Number(n) => write!(f, "{}", n),
            LInt::Add(a, b) => write!(f, "(+ {} {})", a, b),
            LInt::Subtract(a, b) => write!(f, "(- {} {})", a, b),
            LInt::Read() => write!(f, "(read)"),
        }
    }
}

impl std::fmt::Display for LVar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LVar::Let(bindings, body) => {
                write!(f, "(let (")?;
                for binding in bindings {
                    write!(f, "({} {})", binding.name, binding.value)?;
                }
                write!(f, ") {})", body)
            }
            LVar::Var(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(add 2 (subtract 4 2))";
        let expected = vec![
            Token::LParen,
            Token::Identifier("add".to_string()),
            Token::Number(2),
            Token::LParen,
            Token::Identifier("subtract".to_string()),
            Token::Number(4),
            Token::Number(2),
            Token::RParen,
            Token::RParen,
            Token::EndOfFile,
        ];

        assert_eq!(tokenize(input), expected);

        let input = "(let ((x 2) (y 3)) (let ((z (+ x y))) (+ z 1)))";
        let expected = vec![
            Token::LParen,
            Token::Identifier("let".to_string()),
            Token::LParen,
            Token::LParen,
            Token::Identifier("x".to_string()),
            Token::Number(2),
            Token::RParen,
            Token::LParen,
            Token::Identifier("y".to_string()),
            Token::Number(3),
            Token::RParen,
            Token::RParen,
            Token::LParen,
            Token::Identifier("let".to_string()),
            Token::LParen,
            Token::LParen,
            Token::Identifier("z".to_string()),
            Token::LParen,
            Token::Identifier("+".to_string()),
            Token::Identifier("x".to_string()),
            Token::Identifier("y".to_string()),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::LParen,
            Token::Identifier("+".to_string()),
            Token::Identifier("z".to_string()),
            Token::Number(1),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::EndOfFile,
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_parse() {
        let input = "(+ 2 (- 4 2))";
        let tokens = tokenize(input);
        let expected = Expression::LInt(LInt::Add(
            Box::new(Expression::LInt(LInt::Number(2))),
            Box::new(Expression::LInt(LInt::Subtract(
                Box::new(Expression::LInt(LInt::Number(4))),
                Box::new(Expression::LInt(LInt::Number(2))),
            ))),
        ));

        assert_eq!(parse(&tokens), expected);

        // let input = "(- 100 (+ 3 4 (read)))";
        // let tokens = tokenize(input);
        // let expected = Expression::Call(
        //     "-".to_string(),
        //     vec![
        //         Expression::Number(100),
        //         Expression::Call(
        //             "+".to_string(),
        //             vec![
        //                 Expression::Number(3),
        //                 Expression::Number(4),
        //                 Expression::Call("read".to_string(), vec![]),
        //             ],
        //         ),
        //     ],
        // );

        // assert_eq!(parse(&tokens), expected);
    }

    #[test]
    fn test_eval() {
        let input = "(+ 2 (- 4 2))";
        let tokens = tokenize(input);
        let ast = parse(&tokens);
        let expected = 4;

        assert_eq!(ast.evaluate(), expected);

        // let input = "(- 100 (+ 3 4 (read)))";
        // let tokens = tokenize(input);
        // let ast = parse(&tokens);
        // let expected = 89;

        // assert_eq!(ast.eval(), expected);
    }
}
