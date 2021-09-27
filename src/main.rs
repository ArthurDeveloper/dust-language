#![allow(unused_imports)]

pub mod compiler;

use compiler::tokens;
use compiler::parser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::tokens::Symbol;
    use crate::tokens::Token;
    use crate::parser::Parser;

    #[test]
    fn test_symbols() {
        Token::Symbol(Symbol::new("(").unwrap());
        Token::Symbol(Symbol::new(")").unwrap());
        Token::Symbol(Symbol::new("[").unwrap());
        Token::Symbol(Symbol::new("]").unwrap());
        Token::Symbol(Symbol::new("{").unwrap());
        Token::Symbol(Symbol::new("}").unwrap());
        Token::Symbol(Symbol::new(":").unwrap());
        Token::Symbol(Symbol::new(";").unwrap());
    }

    #[test]
    fn test_parser() {
        Parser::from_file("res/test_symbol.dust");
    }

}
