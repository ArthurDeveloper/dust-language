use std::result::{Result};

pub enum Token {
    Identifier(String),
    Literal(String),
    Symbol(Symbol)
}

pub struct Symbol {
    pub symbol: String
} 

impl Symbol {

    fn is_valid(symbol: &str) -> bool {
        let valid_symbols = &[
            "[", "]", "(", ")", "{", "}", ":", ";", "+", "-", "*", ",",
            "**", "~", ".", "=", "+=", "-=", "<=", ">=" 
        ];

        for i in 0..valid_symbols.len() {
            if symbol == valid_symbols[i] {
                return true;
            }  
        }

        return false
    }

    pub fn new(symbol: &str) -> Result<Symbol, String> {
        if !Symbol::is_valid(&symbol.to_owned()) {
            return Err(format!("Invalid token: {}", symbol));
        }

        Ok(
            Symbol {
                symbol: String::from(symbol)
            }
        )
    } 
}
