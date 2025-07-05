mod parser;
mod runtime;

use parser::lexer::Lexer;
use parser::parser::Parser;

use runtime::evaluator::Evaluator;

fn main() {
    let source = r#"
        init greet(name):
            log(`Hello, ${name}`)

        greet("Joe")
        greet("Ada")
    "#;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token.kind == parser::tokens::TokenKind::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut evaluator = Evaluator::new();
    evaluator.evaluate(ast);
}
    