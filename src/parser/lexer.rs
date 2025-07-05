use crate::parser::tokens::{Token, TokenKind};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenKind>,
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '$'
}

fn is_identifier_part(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '$'
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("init".to_string(), TokenKind::Init);
        keywords.insert("if".to_string(), TokenKind::If);
        keywords.insert("elif".to_string(), TokenKind::Elif);
        keywords.insert("else".to_string(), TokenKind::Else);
        keywords.insert("for".to_string(), TokenKind::For);
        keywords.insert("while".to_string(), TokenKind::While);
        keywords.insert("in".to_string(), TokenKind::In);
        keywords.insert("try".to_string(), TokenKind::Try);
        keywords.insert("catch".to_string(), TokenKind::Catch);
        keywords.insert("finally".to_string(), TokenKind::Finally);
        keywords.insert("throw".to_string(), TokenKind::Throw);
        keywords.insert("class".to_string(), TokenKind::Class);
        keywords.insert("extends".to_string(), TokenKind::Extends);
        keywords.insert("fixed".to_string(), TokenKind::Fixed);
        keywords.insert("return".to_string(), TokenKind::Return);
        keywords.insert("log".to_string(), TokenKind::Log);
        keywords.insert("get".to_string(), TokenKind::Get);
        keywords.insert("from".to_string(), TokenKind::From);
        keywords.insert("range".to_string(), TokenKind::Range);
        keywords.insert("secure".to_string(), TokenKind::Secure);
        keywords.insert("true".to_string(), TokenKind::BooleanLiteral(true));
        keywords.insert("false".to_string(), TokenKind::BooleanLiteral(false));
        keywords.insert("null".to_string(), TokenKind::Null);
        keywords.insert("undefined".to_string(), TokenKind::Undefined);

        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let line = self.line;
        let column = self.column;

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF, line, column);
        }

        let c = self.advance();



        match c {
            '(' => self.make_token(TokenKind::LeftParen, line, column),
            ')' => self.make_token(TokenKind::RightParen, line, column),
            '{' => self.make_token(TokenKind::LeftBrace, line, column),
            '}' => self.make_token(TokenKind::RightBrace, line, column),
            '[' => self.make_token(TokenKind::LeftBracket, line, column),
            ']' => self.make_token(TokenKind::RightBracket, line, column),
            ':' => self.make_token(TokenKind::Colon, line, column),
            ';' => self.make_token(TokenKind::Semicolon, line, column),
            ',' => self.make_token(TokenKind::Comma, line, column),
            '.' => self.make_token(TokenKind::Dot, line, column),
            '*' => self.make_token(TokenKind::Star, line, column),
            '+' => self.make_token(TokenKind::Plus, line, column),
            '-' => {
                if self.match_char('>') {
                    self.make_token(TokenKind::Arrow, line, column)
                } else {
                    self.make_token(TokenKind::Minus, line, column)
                }
            }
            '/' => self.make_token(TokenKind::Slash, line, column),
            '%' => self.make_token(TokenKind::Percent, line, column),
            '?' => self.make_token(TokenKind::Question, line, column),
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenKind::DoubleEqual, line, column)
                } else {
                    self.make_token(TokenKind::Equal, line, column)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenKind::NotEqual, line, column)
                } else {
                    self.make_token(TokenKind::NotEqual, line, column)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenKind::GreaterEqual, line, column)
                } else {
                    self.make_token(TokenKind::Greater, line, column)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenKind::LessEqual, line, column)
                } else {
                    self.make_token(TokenKind::Less, line, column)
                }
            }
            '"' | '\'' => self.scan_string(c, line, column),
            '`' => self.scan_template_string(line, column),

            c if c.is_ascii_digit() => self.scan_number(c, line, column),
            c if is_identifier_start(c) => self.scan_identifier(c, line, column),
            '\n' => {
                self.line += 1;
                self.column = 1;
                self.next_token()
            }
            _ => self.make_token(TokenKind::Unknown(c), line, column),
        }
    }

    fn scan_identifier(&mut self, first: char, line: usize, column: usize) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while !self.is_at_end() && is_identifier_part(self.source[self.position]) {
            ident.push(self.advance());
        }

        if let Some(kind) = self.keywords.get(&ident) {
            self.make_token(kind.clone(), line, column)
        } else {
            self.make_token(TokenKind::Identifier(ident), line, column)
        }
    }

    fn scan_number(&mut self, first: char, line: usize, column: usize) -> Token {
        let mut number = String::new();
        number.push(first);

        while !self.is_at_end() && self.source[self.position].is_ascii_digit() {
            number.push(self.advance());
        }

        if !self.is_at_end() && self.source[self.position] == '.' {
            number.push(self.advance());
            while !self.is_at_end() && self.source[self.position].is_ascii_digit() {
                number.push(self.advance());
            }
        }

        self.make_token(TokenKind::NumberLiteral(number), line, column)
    }

    fn scan_string(&mut self, quote: char, line: usize, column: usize) -> Token {
        let mut string = String::new();

        while !self.is_at_end() && self.source[self.position] != quote {
            let ch = self.advance();
            if ch == '\\' && !self.is_at_end() {
                let next = self.advance();
                match next {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    '"' => string.push('"'),
                    '\'' => string.push('\''),
                    '\\' => string.push('\\'),
                    other => string.push(other),
                }
            } else {
                string.push(ch);
            }
        }

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF, line, column); // unterminated
        }

        self.advance(); // consume closing quote
        self.make_token(TokenKind::StringLiteral(string), line, column)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.position];
        self.position += 1;
        self.column += 1;
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.position] != expected {
            return false;
        }
        self.position += 1;
        self.column += 1;
        true
    }

    fn make_token(&self, kind: TokenKind, line: usize, column: usize) -> Token {
        Token { kind, line, column }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            let c = self.source[self.position];
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '<' if self.peek_ahead(1) == '*' => {
                    self.advance(); // <
                    self.advance(); // *
                    while !self.is_at_end() && !(self.peek() == '*' && self.peek_ahead(1) == '>') {
                        self.advance();
                    }
                    if !self.is_at_end() {
                        self.advance(); // *
                        self.advance(); // >
                    }
                }
                _ => break,
            }
        }
    }
    

    fn scan_template_string(&mut self, line: usize, column: usize) -> Token {
        let mut content = String::new();
    
        while !self.is_at_end() && self.source[self.position] != '`' {
            let ch = self.advance();
            if ch == '\\' && !self.is_at_end() {
                // handle escape sequences
                let next = self.advance();
                match next {
                    'n' => content.push('\n'),
                    't' => content.push('\t'),
                    '`' => content.push('`'),
                    '$' => content.push('$'),
                    '\\' => content.push('\\'),
                    other => content.push(other),
                }
            } else {
                content.push(ch);
            }
        }
    
        if self.is_at_end() {
            return self.make_token(TokenKind::EOF, line, column);
        }
    
        self.advance(); // closing backtick
        self.make_token(TokenKind::TemplateString(content), line, column)
    }  
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }
    
    fn peek_ahead(&self, offset: usize) -> char {
        if self.position + offset >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + offset]
        }
    }
    
}
