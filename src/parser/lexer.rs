//! Input tokenizer

use std;
use std::borrow::ToOwned;
use std::rc::Rc;
use parser::tokens::{Token, SourceLocation, dummy_source};
use parser::util::{SharedString, rcstr, rcstring};

// --- Lexer: Error -------------------------------------------------------------
const SYMBOL_CHARS: &'static str = "+-*/%\\=<>!?&_#$§^`.,:@";

pub type LexerResult<T> = Result<T, LexerError>;

pub enum LexerError {
    UnexpectedChar {
        expected: SharedString,
        found: SharedString,
        location: SourceLocation
    },
    UnknownToken {
        token: SharedString,  // result of curr_repr
        location: SourceLocation
    },
    InvalidInteger {
        input: SharedString,
        location: SourceLocation
    }
}

impl std::fmt::Show for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LexerError::UnexpectedChar { ref expected, ref found, ref location } => {
                write!(f, "unexpected character: expected `{}`, but found {} at {}",
                       expected, found, location)
            },
            LexerError::UnknownToken { ref token, ref location } => {
                write!(f, "unknown token: `{}` at {}", token, location)
            },
            LexerError::InvalidInteger { ref input, ref location } => {
                write!(f, "invalid integer: `{}` at {}", input, location)
            }
        }
    }
}

macro_rules! unknown_token(
    ($token:expr @ $location:expr) => (
        return Err(LexerError::UnknownToken {
            token: $token.clone(),
            location: $location
        })
    )
);

macro_rules! invalid_number(
    ($input:expr @ $location:expr) => (
        return Err(LexerError::InvalidInteger {
            input: $input.clone(),
            location: $location
        })
    )
);

// --- Lexer --------------------------------------------------------------------

pub trait Lexer {
    /// Get the source of the current token
    fn get_source(&self) -> SourceLocation;

    /// Get the next token
    fn next_token(&mut self) -> LexerResult<Token>;

    /// Tokenize the input into a vector
    fn tokenize(&mut self) -> LexerResult<Vec<Token>>;
}


/// Lexer for tokenize a file from disk/memory.
pub struct FileLexer<'a> {
    source: &'a str,
    file: SharedString,
    len: uint,
    pos: uint,
    curr: Option<char>,
    lineno: uint
}

impl<'a> FileLexer<'a> {
    pub fn new(source: &'a str, file: &'a str) -> FileLexer<'a> {
        FileLexer {
            source: source,
            file: rcstr(file),
            len: source.len(),
            pos: 0,
            curr: if source.len() > 0 { Some(source.char_at(0)) } else { None },
            lineno: 1
        }
    }

    /// --- Internal methods: Helpers -------------------------------------------

    /// Whether we've reached EOF
    fn is_eof(&self) -> bool {
        self.curr.is_none()
    }

    /// Move on to the next char
    fn bump(&mut self) {
        self.curr = self.nextch();
        self.pos += 1;

        debug!("Moved on to {}", self.curr)
    }

    /// Get the next char if possible
    fn nextch(&self) -> Option<char> {
        let mut new_pos = self.pos + 1;
        // When encountering multi-byte UTF-8, we may stop in the middle
        // of it. Fast forward till we see the next actual char or EOF
        while !self.source.is_char_boundary(new_pos)
            && self.pos < self.len {
            new_pos += 1;
        }

        if new_pos < self.len {
            Some(self.source.char_at(new_pos))
        } else {
            None
        }
    }

    fn expect(&mut self, expect: char) -> LexerResult<()>  {
        if self.curr != Some(expect) {
            let expect_str = String::from_chars(&[expect]).escape_default();
            let found_str = match self.curr {
                Some(_) => format!("'{}'", self.curr_repr()),
                None => "EOF".to_owned()
            };

            return Err(LexerError::UnexpectedChar {
                expected: rcstring(expect_str),
                found: rcstring(found_str),
                location: self.get_source()
            });
        }

       self.bump();

       Ok(())
    }

    /// Get a printable representation of the current char
    fn curr_repr(&self) -> SharedString {
        match self.curr {
            Some(c) => {
                Rc::new(String::from_chars(&[c]).escape_default())
            },
            None => rcstr("EOF")
        }
    }


    /// Collect a series of chars starting at the current character
    fn collect(&mut self, cond: |&char| -> bool) -> SharedString {
        let mut chars = vec![];

        debug!("start colleting");

        while let Some(c) = self.curr {
            if cond(&c) {
                chars.push(c);
                self.bump();
            } else {
                debug!("colleting finished");
                break;
            }
        }

        Rc::new(String::from_chars(&*chars))
    }

    fn eat_all(&mut self, cond: |&char| -> bool) {
        while let Some(c) = self.curr {
            if cond(&c) { self.bump(); }
            else { break; }
        }
    }

    // --- Internal methods: Tokenizers -----------------------------------------

    /// Tokenize a number
    fn tokenize_number(&mut self) -> LexerResult<Token> {
        let sign = if self.curr == Some('-') {
            self.bump();
            -1.0
        } else {
            1.0
        };
        let number = self.collect(|c| c.is_numeric() || *c == '.');

        let number = if let Some(m) = number.parse() { m }
                      else { invalid_number!(number @ self.get_source()) };

        Ok(Token::NUMBER(sign * number))
    }

    /// Tokenize a symbol
    fn tokenize_symbol(&mut self) -> LexerResult<Token> {
        let symbol = self.collect(|c| {
            c.is_alphanumeric() || SYMBOL_CHARS.contains_char(*c)
        });
        Ok(Token::SYMBOL(symbol))
    }

    /// Tokenize a string
    fn tokenize_string(&mut self) -> LexerResult<Token> {
        self.bump();
        let mut string = vec![];
        let mut escaped = false;

        while let Some(c) = self.curr {
            if c == '"' && !escaped {
                break;
            } else {
                string.push(c);
                self.bump();

                escaped = c == '\\';
            }
        }

        try!(self.expect('"'));

        let string = String::from_chars(&*string)
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\\\", "\\")
            .replace("\\\"", "\"")
            .replace("\\0", "\0");

        Ok(Token::STRING(Rc::new(string)))
    }


    /// Read the next token and return it
    fn read_token(&mut self) -> LexerResult<Option<Token>> {
        let c = match self.curr {
            Some(c) => c,
            None => return Ok(Some(Token::EOF))
        };

        let token = match c {
            c if c.is_numeric() => {
                try!(self.tokenize_number())
            },
            c if c.is_alphanumeric() || SYMBOL_CHARS.contains_char(c) => {
                try!(self.tokenize_symbol())
            },
            '"' => {
                try!(self.tokenize_string())
            }
            '(' => { self.bump(); Token::LPAREN },
            ')' => { self.bump(); Token::RPAREN },
            '{' => { self.bump(); Token::LBRACE },
            '}' => { self.bump(); Token::RBRACE },

            ';' => {
                self.eat_all(|c| *c != '\n');
                return Ok(None);
            },
            c if c.is_whitespace() => {
                if c == '\n' { self.lineno += 1; }

                self.bump();
                return Ok(None);
            },
            _ => {
                unknown_token!(self.curr_repr() @ self.get_source())
                // UNKNOWN(format!("{}", c).into_string())
            }
        };

        Ok(Some(token))
    }
}

impl<'a> Lexer for FileLexer<'a> {
    fn get_source(&self) -> SourceLocation {
        SourceLocation {
            filename: self.file.clone(),
            lineno: self.lineno
        }
    }

    fn next_token(&mut self) -> LexerResult<Token> {
        if self.is_eof() {
            Ok(Token::EOF)
        } else {
            let mut tok = try!(self.read_token());
            while tok.is_none() {
                // Token is to be ignored, try next one
                tok = try!(self.read_token());
            }

            Ok(tok.unwrap())  // Can't really be None any more
        }
    }

    #[allow(dead_code)]  // Used for tests
    fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens = vec![];

        // NOTE: We can't use `for c in self.iter` because then we can't
        //       access `self.iter` inside the body because it's borrowed.
        while !self.is_eof() {
            debug!("Processing {}", self.curr);

            if let Some(t) = try!(self.read_token()) {
                tokens.push(t);
            }

            debug!("So far: {}", tokens);
        }

        Ok(tokens)
    }
}

// A dummy lexer that takes it's tokens from a vector. Used for parser testing.
impl Lexer for Vec<Token> {
    fn get_source(&self) -> SourceLocation {
        dummy_source()
    }

    fn next_token(&mut self) -> LexerResult<Token> {
        match self.remove(0) {
            Some(tok) => Ok(tok),
            None => Ok(Token::EOF)
        }
    }

    fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut v = vec![];
        v.push_all(self[]);

        Ok(v)
    }
}


#[cfg(test)]
mod tests {
    use parser::lexer::{Lexer, FileLexer};
    use parser::tokens::Token;
    use parser::tokens::Token::*;
    use parser::util::rcstr;

    fn tokenize(src: &'static str) -> Vec<Token> {
        FileLexer::new(src, "<test>").tokenize().unwrap()
    }

    #[test]
    fn test_symbol() {
        assert_eq!(tokenize("+"),
                   vec![SYMBOL(rcstr("+"))]);
        assert_eq!(tokenize("-"),
                   vec![SYMBOL(rcstr("-"))]);
        assert_eq!(tokenize("*"),
                   vec![SYMBOL(rcstr("*"))]);
        assert_eq!(tokenize("/"),
                   vec![SYMBOL(rcstr("/"))]);
    }

    #[test]
    fn test_number() {
        assert_eq!(tokenize("123"),
                   vec![NUMBER(123.)]);
    }

    /*#[test]
    fn test_number_neg() {
        assert_eq!(tokenize("-123"),
                   vec![NUMBER(-123)]);
    }*/

    #[test]
    fn test_parens() {
        assert_eq!(tokenize("("),
                   vec![LPAREN]);
        assert_eq!(tokenize(")"),
                   vec![RPAREN]);
    }
}