use parser::{parse, tokenize};

mod parser;

fn main() {
    let input = "(let ((x 2) (y 3)) (let ((z (+ x y))) (+ z (read))))";

    // Check if parens are balanced
    let mut parens = 0;
    for c in input.chars() {
        if c == '(' {
            parens += 1;
        } else if c == ')' {
            parens -= 1;
        }
    }
    if parens != 0 {
        println!("Unbalanced parens!!!");
    }

    let tokens = tokenize(input);
    let ast = parse(&tokens);

    println!("AST: {}", &ast);

    // println!("Partially evaluated AST: {}", visualize(&pe_ast));

    println!("{}", ast.evaluate());
}
