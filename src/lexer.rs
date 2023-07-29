use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers and literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

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
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

// public API
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
        self.skip_whitespace();

        let token = match self.ch {
            Some(t) => match t {
                '=' => {
                    let ch = self.peek();
                    if ch.is_some() && ch.unwrap() == '=' {
                        self.read_char();
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    let ch = self.peek();
                    if ch.is_some() && ch.unwrap() == '=' {
                        self.read_char();
                        Token::NotEqual
                    } else {
                        Token::Bang
                    }
                }
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '<' => Token::LessThan,
                '>' => Token::GreaterThan,
                ',' => Token::Comma,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '/' => Token::Slash,
                '*' => Token::Asterisk,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_ident();
                    return Ok(match ident.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Ident(ident),
                    });
                }
                '0'..='9' => {
                    return Ok(Token::Int(self.read_int()));
                }
                _ => Token::Illegal,
            },
            None => Token::EOF,
        };

        self.read_char();
        Ok(token)
    }
}

// private helpers
impl Lexer {
    fn read_char(&mut self) {
        if let Some(ch) = self.input.chars().nth(self.read_position) {
            self.position = self.read_position;
            self.read_position += 1;
            self.ch = Some(ch);
        } else {
            self.ch = None;
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let mut out = String::new();
        while self.ch.is_some() && Self::is_ident_char(self.ch.unwrap()) {
            out.push(self.ch.unwrap());
            self.read_char();
        }
        out
    }

    fn is_ident_char(ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn read_int(&mut self) -> i32 {
        let mut out = String::new();
        while self.ch.is_some() && self.ch.unwrap().is_ascii_digit() {
            out.push(self.ch.unwrap());
            self.read_char();
        }
        out.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn it_works() -> Result<()> {
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
            println!("expected: {token:?}, received {next_token:?}");
            assert_eq!(token, next_token);
        }

        Ok(())
    }

    #[test]
    fn it_works_better() -> Result<()> {
        let source = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#
        .to_string();

        let mut lexer = Lexer::new(source);

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::EOF,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {token:?}, received {next_token:?}");
            assert_eq!(token, next_token);
        }

        Ok(())
    }
}
