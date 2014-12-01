//! Tokens

use std::fmt;
use parser::util::{rcstr, SharedString};

// --- Token --------------------------------------------------------------------

/// A token
#[deriving(Clone, PartialEq, Eq)]
pub enum Token {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    SYMBOL(SharedString),
    INTEGER(i64),

    EOF,
    PLACEHOLDER
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LPAREN        => write!(f, "("),
            Token::RPAREN        => write!(f, ")"),
            Token::LBRACE        => write!(f, "{{"),
            Token::RBRACE        => write!(f, "}}"),

            Token::SYMBOL(ref s) => write!(f, "{}", s),
            Token::INTEGER(i)    => write!(f, "{}", i),

            Token::EOF           => write!(f, "EOF"),
            Token::PLACEHOLDER   => write!(f, "PLACEHOLDER")
        }
    }
}


// --- Source location ----------------------------------------------------------

/// Represntation of a location in the input
#[deriving(PartialEq, Eq, Clone)]
pub struct SourceLocation {
    pub filename: SharedString,
    pub lineno: uint
}

impl fmt::Show for SourceLocation {
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