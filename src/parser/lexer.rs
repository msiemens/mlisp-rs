use std::rc::Rc;
use parser::tokens::{Token, SourceLocation, dummy_source};
use parser::util::{SharedString, rcstr, rcstring, fatal};


pub trait Lexer {
    fn get_source(&self) -> SourceLocation;
    fn next_token(&mut self) -> Token;
    fn tokenize(&mut self) -> Vec<Token>;
}


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
            curr: Some(source.char_at(0)),
            lineno: 1
        }
    }


    fn fatal(&self, msg: String) -> ! {
        fatal(msg, &self.get_source())
    }


    fn is_eof(&self) -> bool {
        self.curr.is_none()
    }

    fn bump(&mut self) {
        self.curr = self.nextch();
        self.pos += 1;

        debug!("Moved on to {}", self.curr)
    }

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

    //fn nextch_is(&self, c: char) -> bool {
    //    self.nextch() == Some(c)
    //}

    //fn expect(&mut self, expect: char) {
    //    if self.curr != Some(expect) {
    //        let expect_str = match expect {
    //            '\'' => "quote".into_string(),
    //            c => format!("'{}'", c)
    //        };
    //        let found_str = match self.curr {
    //            Some(_) => format!("'{}'", self.curr_repr()),
    //            None => "EOF".into_string()
    //        };
    //
    //        self.fatal(format!("Expected `{}`, found `{}`",
    //                           expect_str, found_str))
    //    }
    //
    //   self.bump();
    //}

    //fn curr_repr(&self) -> SharedString {
    //    match self.curr {
    //        Some(c) => {
    //            let mut repr = vec![];
    //            c.escape_default(|r| repr.push(r));
    //            Rc::new(String::from_chars(repr[]))
    //        },
    //        None => rcstr("EOF")
    //    }
    //}


    /// Collect a series of chars starting at the current character
    fn collect(&mut self, cond: |&char| -> bool) -> SharedString {
        let mut chars = vec![];

        debug!("start colleting")

        while let Some(c) = self.curr {
            if cond(&c) {
                chars.push(c);
                self.bump();
            } else {
                debug!("colleting finished")
                break;
            }
        }

        Rc::new(String::from_chars(chars[]))
    }

    //fn eat_all(&mut self, cond: |&char| -> bool) {
    //    while let Some(c) = self.curr {
    //        if cond(&c) { self.bump(); }
    //        else { break; }
    //    }
    //}


    fn tokenize_number(&mut self) -> Token {
        let sign = if self.curr == Some('-') {
            self.bump();
            -1
        } else {
            1
        };
        let integer = self.collect(|c| c.is_digit());

        let integer = if let Some(m) = from_str(integer[]) {
            m
        } else {
            self.fatal(format!("invalid integer: `{}`", integer))
        };

        Token::INTEGER(sign * integer)
    }


    /// Read the next token and return it
    fn read_token(&mut self) -> Option<Token> {
        let c = match self.curr {
            Some(c) => c,
            None => return Some(Token::EOF)
        };

        let token = match c {
            c if c.is_digit() => {
                self.tokenize_number()
            },
            '+' | '-' | '*' | '/' => {
                let is_num = c == '-' && match self.nextch() {
                    Some(c) => c.is_digit(),
                    None => false
                };

                if is_num {
                    // Tokenize number
                    self.tokenize_number()
                } else {
                    self.bump();
                    Token::SYMBOL(rcstring(String::from_chars(&[c])))
                }
            },
            '(' => { self.bump(); Token::LPAREN },
            ')' => { self.bump(); Token::RPAREN },

            c if c.is_whitespace() => {
                if c == '\n' { self.lineno += 1; }

                self.bump();
                return None;
            },
            c => {
                self.fatal(format!("unknown token: {}", c))
                // UNKNOWN(format!("{}", c).into_string())
            }
        };

        Some(token)
    }
}

impl<'a> Lexer for FileLexer<'a> {
    fn get_source(&self) -> SourceLocation {
        SourceLocation {
            filename: self.file.clone(),
            lineno: self.lineno
        }
    }

    fn next_token(&mut self) -> Token {
        if self.is_eof() {
            Token::EOF
        } else {
            let mut tok = self.read_token();
            while tok.is_none() {
                // Token is to be ignored, try next one
                tok = self.read_token();
            }

            tok.unwrap()  // Can't really be None any more
        }
    }

    #[allow(dead_code)]  // Used for tests
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        // NOTE: We can't use `for c in self.iter` because then we can't
        //       access `self.iter` inside the body because it's borrowed.
        while !self.is_eof() {
            debug!("Processing {}", self.curr)

            if let Some(t) = self.read_token() {
                tokens.push(t);
            }

            debug!("So far: {}", tokens)
        }

        tokens
    }
}

// A dummy lexer that takes it's tokens from a vector. Used for parser testing.
impl Lexer for Vec<Token> {
    fn get_source(&self) -> SourceLocation {
        dummy_source()
    }

    fn next_token(&mut self) -> Token {
        match self.remove(0) {
            Some(tok) => tok,
            None => Token::EOF
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut v = vec![];
        v.push_all(self[]);

        v
    }
}


#[cfg(test)]
mod tests {
    use parser::lexer::{Lexer, FileLexer};
    use parser::tokens::Token;
    use parser::tokens::Token::*;
    use parser::util::rcstr;

    fn tokenize(src: &'static str) -> Vec<Token> {
        FileLexer::new(src, "<test>").tokenize()
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
                   vec![INTEGER(123)]);
    }

    #[test]
    fn test_number_neg() {
        assert_eq!(tokenize("-123"),
                   vec![INTEGER(-123)]);
    }

    #[test]
    fn test_parens() {
        assert_eq!(tokenize("("),
                   vec![LPAREN]);
        assert_eq!(tokenize(")"),
                   vec![RPAREN]);
    }
}