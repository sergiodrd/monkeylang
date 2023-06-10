use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers and literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut l = Self {
            input: source,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Result<Token> {
        let token = match self.ch {
            Some(t) => {
                match t {
                    '=' => Token::Assign,
                    ';' => Token::Semicolon,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    ',' => Token::Comma,
                    '+' => Token::Plus,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    _ => Token::Illegal
                }
            }
            None => Token::EOF
        };

        self.read_char();
        Ok(token)
    }
}

impl Lexer {
    fn read_char(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.read_position);

        if let Some(ch) = c {
            self.position = self.read_position;
            self.read_position += 1;
            self.ch = Some(ch);
        } else {
            self.ch = None;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, Ok};

    #[test]
    fn it_works() -> Result<()>{
        let mut lexer = Lexer::new("=+(){},;".to_string());

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        } 

        Ok(())
    }
}
