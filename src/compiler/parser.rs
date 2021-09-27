#![allow(dead_code)]

use crate::tokens::Token;
use crate::tokens::Symbol;
use std::fs;

pub struct Parser {

}

impl Parser {
    pub fn from_string(string: String) {
        let mut output: String = String::new();
        for char in string.chars() {
            let str_data = &format!("{}", char).to_owned(); 
            output = format!(
                "{}{}",
                output, 
                match Symbol::new(str_data) {
                    Ok(t) => format!("{}", char),
                    Err(msg) => panic!(msg)
                }
            );
        }        
    }

    pub fn from_file(path: &str) {
        let data = fs::read_to_string(path).expect(&format!("Error while trying to openg the file {}", path).to_owned());
        Parser::from_string(data);
    }
}
