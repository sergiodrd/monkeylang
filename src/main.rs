use anyhow::Result;
use monkeylang::lexer::{Lexer, Token};
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("Welcome to the monkeylang repl.");

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut lexer = Lexer::new(input);
        let mut current_token = lexer.next_token()?;
        while current_token != Token::EOF {
            println!("{current_token:?}");
            current_token = lexer.next_token()?;
        }
    }
}
