use std::collections::DList;
use parser::ast::{Expr, ExprNode};
use parser::tokens::{Token, SourceLocation};
use parser::lexer::{Lexer, FileLexer};
use parser::util::fatal;

mod util;
mod tokens;
pub mod ast;
mod lexer;


pub struct Parser<'a> {
    location: SourceLocation,
    token: Token,
    buffer: DList<Token>,
    lexer: Box<Lexer + 'a>
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, file: &'a str) -> Parser<'a> {
        Parser::with_lexer(box FileLexer::new(source, file))
    }

    pub fn with_lexer(lx: Box<Lexer + 'a>) -> Parser<'a> {
        let mut lx = lx;

        Parser {
            token: lx.next_token(),
            location: lx.get_source(),
            buffer: DList::new(),
            lexer: lx
        }
    }


    fn fatal(&self, msg: String) -> ! {
        fatal(msg, &self.location);
    }

    fn unexpected_token(&self, tok: &Token, expected: Option<&'static str>) -> ! {
        match expected {
            Some(ex) => self.fatal(format!("unexpected token: `{}`, expected {}", tok, ex)),
            None => self.fatal(format!("unexpected token: `{}`", tok))
        }
    }

    fn bump(&mut self) {
        self.token = match self.buffer.pop_front() {
            Some(tok) => tok,
            None => self.lexer.next_token()
        };
    }

    fn update_location(&mut self) -> SourceLocation {
        self.location = self.lexer.get_source();
        self.location.clone()
    }

    fn eat(&mut self, tok: &Token) -> bool {
        if self.token == *tok {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, tok: &Token) {
        if !self.eat(tok) {
            self.fatal(format!("expected `{}`, found `{}`", tok, self.token))
        }
    }

    pub fn look_ahead<R>(&mut self, distance: uint, f: |&Token| -> R) -> R {
        if self.buffer.len() < distance {
            for _ in range(0, distance - self.buffer.len()) {
                self.buffer.push_back(self.lexer.next_token());
            }
        }

        f(self.buffer.iter().nth(distance - 1).unwrap())
    }


    pub fn parse(&mut self) -> ExprNode {
        let location = self.update_location();
        let mut values = vec![];

        debug!("Starting parsing")

        while self.token != Token::EOF {
            values.push(self.parse_expr());
        }

        debug!("Parsing finished")

        // Wrap everything in an SExpr, if what we parsed isn't already one
        if values.len() == 1 {
            values.pop().unwrap()
        } else {
            ExprNode::new(Expr::SExpr(values), location)
        }
    }


    fn parse_number(&mut self) -> ExprNode {
        let location = self.update_location();

        let number = match self.token {
            Token::INTEGER(i) => Expr::Number(i),
            _ => self.unexpected_token(&self.token, Some("a number"))
        };
        self.bump();

        ExprNode::new(number, location)
    }

    fn parse_symbol(&mut self) -> ExprNode {
        let location = self.update_location();

        let symbol = match self.token {
            Token::SYMBOL(ref s) => Expr::Symbol(s.clone()),
            _ => self.unexpected_token(&self.token, Some("a symbol"))
        };
        self.bump();

        ExprNode::new(symbol, location)
    }

    fn parse_sexpr(&mut self) -> ExprNode {
        let location = self.update_location();

        self.expect(&Token::LPAREN);

        let mut exprs = vec![];
        while self.token != Token::RPAREN {
            exprs.push(self.parse_expr());
        }

        self.expect(&Token::RPAREN);

        ExprNode::new(Expr::SExpr(exprs), location)
    }

    fn parse_expr(&mut self) -> ExprNode {
        let stmt = match self.token {
            Token::INTEGER(_) => self.parse_number(),
            Token::SYMBOL(_)  => self.parse_symbol(),
            Token::LPAREN     => self.parse_sexpr(),

            ref tok => self.unexpected_token(tok, Some("a statement"))
        };

        stmt
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use parser::ast::*;
    use parser::tokens::dummy_source;
    use parser::tokens::Token;
    use parser::tokens::Token::*;
    use parser::lexer::Lexer;
    use parser::util::rcstr;

    use super::*;

    fn parse<'a, T>(toks: Vec<Token>, f: |&mut Parser<'a>| -> T) -> T {
        f(&mut Parser::with_lexer(box toks as Box<Lexer>))
    }

    #[test]
    fn test_expr() {
        assert_eq!(
            parse(
                vec![LPAREN, SYMBOL(rcstr("+")), INTEGER(3), INTEGER(2), RPAREN],
                |p| p.parse()
            ),
            ExprNode::new(
                Expr::SExpr(
                    vec![
                        ExprNode::new(
                            Expr::Symbol(rcstr("+")),
                            dummy_source()
                        ),
                        ExprNode::new(
                            Expr::Number(3),
                            dummy_source()
                        ),
                        ExprNode::new(
                            Expr::Number(2),
                            dummy_source()
                        ),
                    ]
                ),
                dummy_source()
            )
        )
    }
}