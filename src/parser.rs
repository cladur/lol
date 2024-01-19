#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    LParen,
    RParen,
    EndOfFile,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(i32),
    Call(String, Vec<Expression>),
}

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
        Some(&Token::LParen) => parse_call(tokens),
        _ => panic!("Unexpected token"),
    }
}

fn parse_number(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Expression {
    match tokens.next() {
        Some(Token::Number(n)) => Expression::Number(*n),
        _ => panic!("Expected number"),
    }
}

fn parse_call(tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>) -> Expression {
    match tokens.next() {
        Some(Token::LParen) => (),
        _ => panic!("Expected '('"),
    }

    let id = match tokens.next() {
        Some(Token::Identifier(id)) => id.to_string(),
        _ => panic!("Expected identifier"),
    };

    let mut args = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::RParen) => break,
            Some(_) => args.push(parse_expression(tokens)),
            None => panic!("Expected ')'"),
        }
    }

    match tokens.next() {
        Some(Token::RParen) => (),
        _ => panic!("Expected ')'"),
    }

    Expression::Call(id, args)
}

pub fn visualize(exp: &Expression) -> String {
    match exp {
        Expression::Number(n) => n.to_string(),
        Expression::Call(id, args) => {
            let mut result = format!("({}", id);
            if !args.is_empty() {
                result += " ";
            }
            for (i, arg) in args.iter().enumerate() {
                result += &visualize(arg);
                if i < args.len() - 1 {
                    result += " ";
                }
            }
            result += ")";
            result
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

        let input = "(+ 2 (- 4 2))";
        let expected = vec![
            Token::LParen,
            Token::Identifier("+".to_string()),
            Token::Number(2),
            Token::LParen,
            Token::Identifier("-".to_string()),
            Token::Number(4),
            Token::Number(2),
            Token::RParen,
            Token::RParen,
            Token::EndOfFile,
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_parse() {
        let input = "(add 2 (subtract 4 2))";
        let tokens = tokenize(input);
        let expected = Expression::Call(
            "add".to_string(),
            vec![
                Expression::Number(2),
                Expression::Call(
                    "subtract".to_string(),
                    vec![Expression::Number(4), Expression::Number(2)],
                ),
            ],
        );

        assert_eq!(parse(&tokens), expected);

        let input = "(- 100 (+ 3 4 (read)))";
        let tokens = tokenize(input);
        let expected = Expression::Call(
            "-".to_string(),
            vec![
                Expression::Number(100),
                Expression::Call(
                    "+".to_string(),
                    vec![
                        Expression::Number(3),
                        Expression::Number(4),
                        Expression::Call("read".to_string(), vec![]),
                    ],
                ),
            ],
        );

        assert_eq!(parse(&tokens), expected);
    }
}
