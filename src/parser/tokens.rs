//! Tokens

use std::fmt;
use parser::util::{rcstr, SharedString};

// --- Token --------------------------------------------------------------------

/// A token
#[derive(Clone, PartialEq)]
pub enum Token {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    STRING(SharedString),
    SYMBOL(SharedString),
    NUMBER(f64),

    EOF,
    PLACEHOLDER
}

impl fmt::String for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LPAREN        => write!(f, "("),
            Token::RPAREN        => write!(f, ")"),
            Token::LBRACE        => write!(f, "{{"),
            Token::RBRACE        => write!(f, "}}"),

            Token::STRING(ref s) => write!(f, "\"{}\"", s.escape_default()),
            Token::SYMBOL(ref s) => write!(f, "{}", s),
            Token::NUMBER(n)     => write!(f, "{}", n),

            Token::EOF           => write!(f, "EOF"),
            Token::PLACEHOLDER   => write!(f, "PLACEHOLDER")
        }
    }
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}


// --- Source location ----------------------------------------------------------

/// Represntation of a location in the input
#[derive(PartialEq, Eq, Clone)]
pub struct SourceLocation {
    pub filename: SharedString,
    pub lineno: usize
}

impl fmt::String for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.filename, self.lineno)
    }
}


/// A dummy source location
pub fn dummy_source() -> SourceLocation {
    SourceLocation {
        filename: rcstr("<input>"),
        lineno: 0
    }
}