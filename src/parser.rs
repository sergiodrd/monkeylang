use crate::ast;
use crate::lexer::{Lexer, Token};
use anyhow::Result;

pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

#[derive(Debug, Clone)]
pub enum ParserError {
    Empty,
}

// public API
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Self {
            lexer,
            curr_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, Vec<ParserError>> {
        let mut statements: Vec<ast::Statement> = vec![];

        while self.curr_token != Token::EOF {
            match self.parse_statement() {
                Some(s) => statements.push(s),
                None => return Err(self.errors.clone()),
            }
            self.next_token();
        }

        Ok(ast::Program { statements })
    }
}

// recursive descent methods
impl Parser {
    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token {
            Token::Let => self.parse_let().map(ast::Statement::Let),
            _ => todo!(), // expected statement, found curr_token
        }
    }

    fn parse_let(&mut self) -> Option<ast::LetStatement> {
        if !self.expect_peek(Token::Ident(String::new())) {
            return None;
        }

        let name = if let Token::Ident(s) = self.curr_token.clone() {
            ast::Ident(s)
        } else {
            unreachable!() // we would've returned early
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        // TODO: actually parse expressions
        while self.curr_token != Token::Semicolon {
            self.next_token();
        }
        let value = ast::Expression::Empty;

        Some(ast::LetStatement {
            name,
            value,
        })
    }
}

// private helpers
impl Parser {
    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().unwrap();
    }

    fn expect_peek(&mut self, tok: Token) -> bool {
        if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&tok) {
            self.next_token();
            true
        } else {
            // TODO: add error, expected next token to be tok, got peek_token instead
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;

    use super::*;
    use anyhow::Result;

    #[test]
    fn let_statements_parse() -> Result<()> {
        let source = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;
        let mut parser = Parser::new(Lexer::new(source.to_string()));
        let program = parser.parse_program().unwrap();
        assert_eq!(
            program,
            Program {
                statements: vec![
                    Statement::Let(LetStatement {
                        name: Ident("x".to_string()),
                        value: Expression::Empty
                    }),
                    Statement::Let(LetStatement {
                        name: Ident("y".to_string()),
                        value: Expression::Empty
                    }),
                    Statement::Let(LetStatement {
                        name: Ident("foobar".to_string()),
                        value: Expression::Empty
                    }),
                ]
            }
        );

        Ok(())
    }
}
