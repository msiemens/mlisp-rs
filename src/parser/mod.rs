//! The Parser

use std;
use std::borrow::ToOwned;
use std::collections::DList;
use parser::ast::{Expr, ExprNode};
use parser::tokens::{Token, SourceLocation};
use parser::lexer::{Lexer, FileLexer, LexerError};

pub mod util;
pub mod tokens;
pub mod ast;
pub mod lexer;

// --- Parser: Error ------------------------------------------------------------

pub type ParserResult<T> = Result<T, ParserError>;

pub enum ParserError {
    UnexpectedToken {
        found: Token,
        expected: Option<String>,
        location: SourceLocation
    },
    FromLexer(LexerError)
}

impl std::fmt::Show for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParserError::UnexpectedToken { ref found, ref expected, ref location } => {
                match *expected {
                    Some(ref expected) => {
                        write!(f, "expected {}, found `{}` at {}", expected, found, location)
                    },
                    None => {
                        write!(f, "unexpected token: `{}` at {}", found, location)
                    }
                }
            },
            ParserError::FromLexer(ref lxerr) => write!(f, "{}", lxerr)
        }
    }
}

impl std::error::FromError<LexerError> for ParserError {
    fn from_error(err: LexerError) -> ParserError {
        ParserError::FromLexer(err)
    }
}

macro_rules! unexpected(
    ($token:expr @ $location:stmt) => (
        return Err(ParserError::UnexpectedToken {
            found: $token.clone(),
            expected: None,
            location: $location
        })
    );

    ($token:expr instead of $msg:expr @ $location:expr) => (
        return Err(ParserError::UnexpectedToken {
            found: $token.clone(),
            expected: Some($msg.to_owned()),
            location: $location
        })
    );

    ($token:expr instead of token: $exp_token:expr @ $location:expr) => (
        return Err(ParserError::UnexpectedToken {
            found: $token.clone(),
            expected: Some(format!("`{}`", $exp_token)),
            location: $location
        })
    );
);

// --- Parser -------------------------------------------------------------------

/// Lispy Parser
pub struct Parser<'a> {
    location: SourceLocation,
    token: Token,
    buffer: DList<Token>,
    lexer: Box<Lexer + 'a>
}

impl<'a> Parser<'a> {

    // Note: Constructors are private!

    fn new(source: &'a str, file: &'a str) -> ParserResult<Parser<'a>> {
        Parser::with_lexer(box FileLexer::new(source, file))
    }

    fn with_lexer(lx: Box<Lexer + 'a>) -> ParserResult<Parser<'a>> {
        let mut lx = lx;

        Ok(Parser {
            token: try!(lx.next_token()),
            location: lx.get_source(),
            buffer: DList::new(),
            lexer: lx
        })
    }

    // --- Internal methods -----------------------------------------------------

    /// Move on to the next token
    fn bump(&mut self) -> ParserResult<()> {
        self.token = match self.buffer.pop_front() {
            Some(tok) => tok,
            None => try!(self.lexer.next_token())
        };
        Ok(())
    }

    /// Update the current source location
    fn update_location(&mut self) -> SourceLocation {
        self.location = self.lexer.get_source();
        self.location.clone()
    }

    /// Expect the current token to be `tok` and continue or fail
    fn expect(&mut self, tok: &Token) -> ParserResult<()> {
        if self.token == *tok {
            try!(self.bump());
            Ok(())
        } else {
            unexpected!(self.token instead of token: tok @ self.location.clone())
        }
    }

    // --- Public methods -------------------------------------------------------

    /// Parse all the input
    pub fn parse(source: &'a str, file: &'a str) -> ParserResult<ExprNode> {
        let mut parser = try!(Parser::new(source, file));
        let location = parser.update_location();
        let mut values = vec![];

        debug!("Starting parsing");

        while parser.token != Token::EOF {
            values.push(try!(parser.parse_expr()));
        }

        debug!("Parsing finished");

        // Wrap everything in an SExpr, if what we parsed isn't already one
        if values.len() == 1 {
            Ok(values.pop().unwrap())
        } else {
            Ok(ExprNode::new(Expr::SExpr(values), location))
        }
    }


    /// Parse a number
    fn parse_number(&mut self) -> ParserResult<ExprNode> {
        let location = self.update_location();

        let number = match self.token {
            Token::NUMBER(i) => Expr::Number(i),
            _ => unexpected!(self.token instead of "a number" @ location)
        };
        try!(self.bump());

        Ok(ExprNode::new(number, location))
    }

    /// Parse a string
    fn parse_string(&mut self) -> ParserResult<ExprNode> {
        let location = self.update_location();

        let string = match self.token {
            Token::STRING(ref s) => Expr::String(s.clone()),
            _ => unexpected!(self.token instead of "a string" @ location)
        };
        try!(self.bump());

        Ok(ExprNode::new(string, location))
    }

    /// Parse a symbol
    fn parse_symbol(&mut self) -> ParserResult<ExprNode> {
        let location = self.update_location();

        let symbol = match self.token {
            Token::SYMBOL(ref s) => Expr::Symbol(s.clone()),
            _ => unexpected!(self.token instead of "a symbol" @ location)
        };
        try!(self.bump());

        Ok(ExprNode::new(symbol, location))
    }

    /// Parse a SExpr
    fn parse_sexpr(&mut self) -> ParserResult<ExprNode> {
        let location = self.update_location();

        try!(self.expect(&Token::LPAREN));

        let mut exprs = vec![];
        while self.token != Token::RPAREN {
            let expr = match self.parse_expr() {
                Ok(expr) => expr,
                Err(err) => return Err(err)
            };
            exprs.push(expr);
        }

        try!(self.expect(&Token::RPAREN));

        Ok(ExprNode::new(Expr::SExpr(exprs), location))
    }

    /// Parse a QExpr
    fn parse_qexpr(&mut self) -> ParserResult<ExprNode> {
        let location = self.update_location();

        try!(self.expect(&Token::LBRACE));

        let mut exprs = vec![];
        while self.token != Token::RBRACE {
            let expr = match self.parse_expr() {
                Ok(expr) => expr,
                Err(err) => return Err(err)
            };
            exprs.push(expr);
        }

        try!(self.expect(&Token::RBRACE));

        Ok(ExprNode::new(Expr::QExpr(exprs), location))
    }

    /// Parse a single expression
    fn parse_expr(&mut self) -> ParserResult<ExprNode> {
        let stmt = match self.token {
            Token::NUMBER(_) => try!(self.parse_number()),
            Token::STRING(_)  => try!(self.parse_string()),
            Token::SYMBOL(_)  => try!(self.parse_symbol()),
            Token::LPAREN     => try!(self.parse_sexpr()),
            Token::LBRACE     => try!(self.parse_qexpr()),

            _ => unexpected!(self.token instead of "an expression" @ self.location.clone())
        };

        Ok(stmt)
    }
}


#[cfg(test)]
mod tests {
    use parser::ast::*;
    use parser::tokens::dummy_source;
    use parser::tokens::Token;
    use parser::tokens::Token::*;
    use parser::lexer::Lexer;
    use parser::util::rcstr;

    use super::*;

    fn parse<'a, T>(toks: Vec<Token>, f: |&mut Parser<'a>| -> T) -> T {
        f(&mut Parser::with_lexer(box toks as Box<Lexer>).unwrap())
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            parse(
                vec![LPAREN, SYMBOL(rcstr("+")), NUMBER(3.), NUMBER(2.), RPAREN],
                |p| p.parse_expr().unwrap()
            ),
            ExprNode::new(
                Expr::SExpr(
                    vec![
                        ExprNode::new(
                            Expr::Symbol(rcstr("+")),
                            dummy_source()
                        ),
                        ExprNode::new(
                            Expr::Number(3.),
                            dummy_source()
                        ),
                        ExprNode::new(
                            Expr::Number(2.),
                            dummy_source()
                        ),
                    ]
                ),
                dummy_source()
            )
        )
    }
}