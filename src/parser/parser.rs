use crate::parser::ast::{Stmt, Expr};
use crate::parser::tokens::TokenKind;

use crate::parser::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.match_token(TokenKind::Init) {
                statements.push(self.parse_init());
            } else if self.match_token(TokenKind::Log) {
                statements.push(self.parse_log());
            } else if matches!(self.peek().kind, TokenKind::Identifier(_)) {
                statements.push(self.parse_expression_stmt());
            } else {
                panic!("Unexpected top-level token: {:?}", self.peek().kind);
            }
        }

        statements
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn match_token(&mut self, expected: TokenKind) -> bool {
        if self.check(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        &self.peek().kind == expected
    }

    fn parse_init(&mut self) -> Stmt {
        let name = if let TokenKind::Identifier(name) = &self.advance().kind {
            name.clone()
        } else {
            panic!("Expected function name after `init`");
        };

        self.expect(TokenKind::LeftParen, "Expected '(' after function name");

        let mut params = Vec::new();
        while !self.check(&TokenKind::RightParen) {
            if let TokenKind::Identifier(param) = &self.advance().kind {
                params.push(param.clone());
            } else {
                panic!("Expected parameter name");
            }

            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(TokenKind::RightParen, "Expected ')' after parameters");
        self.expect(TokenKind::Colon, "Expected ':' after function signature");

        let mut body = Vec::new();
        if self.match_token(TokenKind::Log) {
            body.push(self.parse_log());
        }

        Stmt::Init {
            name,
            params,
            body,
        }
    }

    fn parse_log(&mut self) -> Stmt {
        self.expect(TokenKind::LeftParen, "Expected '(' after `log`");

        let expr = self.parse_expression();

        self.expect(TokenKind::RightParen, "Expected ')' after expression");

        Stmt::Log { value: expr }
    }

    fn parse_expression_stmt(&mut self) -> Stmt {
        let expr = self.parse_expression();
        Stmt::ExprStmt(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        let token = self.advance();
        match &token.kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();

                if self.check(&TokenKind::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();

                    while !self.check(&TokenKind::RightParen) {
                        args.push(self.parse_expression());
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    self.expect(TokenKind::RightParen, "Expected ')' after arguments");

                    Expr::Call {
                        callee: name,
                        arguments: args,
                    }
                } else {
                    Expr::Variable(name)
                }
            }
            TokenKind::StringLiteral(value) => Expr::Literal(value.clone()),
            TokenKind::TemplateString(value) => Expr::Template(value.clone()),
            _ => {
                panic!("Unexpected token in expression: {:?}", token.kind);
            }
        }
    }

    fn expect(&mut self, expected: TokenKind, message: &str) {
        if !self.check(&expected) {
            panic!("{} (found {:?})", message, self.peek().kind);
        }
        self.advance();
    }
}
