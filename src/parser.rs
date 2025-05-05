use crate::lexer::Lexer;
use crate::utils::*;

use std::iter::Peekable;
use crate::lexer;
use crate::lexer::*;
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    lifetime:usize,
}

#[derive(Debug)]
pub enum Error {
    EndOfFile,
    Lexer(lexer::Error),
    Unexpected(lexer::Token),
}

pub type ParseResult<T> = Result<T, Error>;

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str) -> Parser<'a> {
        eprintln!("[Parser::new] initializing parser");
        Parser { lexer: Lexer::new(contents).peekable(), lifetime:0 }
    }

    pub fn new_lifetime(&mut self){
        self.lifetime+=1;

    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        let result = match self.lexer.next() {
            Some(Ok(token)) => {
                eprintln!("[next_token] got token: {:?}", token);
                Ok(token)
            }
            Some(Err(err)) => {
                eprintln!("[next_token] lexer error: {:?}", err);
                Err(Error::Lexer(err))
            }
            None => {
                eprintln!("[next_token] reached end of file");
                Err(Error::EndOfFile)
            }
        };
        result
    }

    pub fn next_token_match(&mut self, t: Token) -> ParseResult<Token> {
        eprintln!("[next_token_match] expecting token: {:?}", t);
        let token = self.next_token()?;
        eprintln!("[next_token_match] got token: {:?}", token);
        if token != t {
            eprintln!("[next_token_match] unexpected token, expected {:?}", t);
            Err(Error::Unexpected(token))
        } else {
            Ok(token)
        }
    }

    pub fn next_token_var(&mut self) -> ParseResult<String> {
        eprintln!("[next_token_var] expecting variable");
        let token = self.next_token()?;
        eprintln!("[next_token_var] got token: {:?}", token);
        match token {
            Token::Var(ident) => Ok(ident.clone()),
            _ => Err(Error::Unexpected(token)),
        }
    }

    pub fn peek_token(&mut self) -> Result<&Token, Error> {
        eprintln!("[peek_token] peeking next token");
        match self.lexer.peek() {
            Some(Ok(token)) => {
                eprintln!("[peek_token] sees token: {:?}", token);
                Ok(token)
            }
            _ => {
                eprintln!("[peek_token] reached end of file when peeking");
                Err(Error::EndOfFile)
            }
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> ParseResult<Expr> {
        eprintln!("[parse_block] entering block parse");
        self.next_token_match(Token::Lbracket)?;
        let mut statements = Vec::new();
        while *self.peek_token()? != Token::Rbracket {
            eprintln!("[parse_block] parsing statement, peek: {:?}", self.peek_token());
            let temp = self.parse_stmt()?;
            eprintln!("[parse_block] parsed statement: {:?}", temp);
            statements.push(temp);
        }
        eprintln!("[parse_block] parsing tail expression");
        if let (Stmt::Expr(last)) = &statements[statements.len()-1]{

            let Some(Stmt::Expr(last)) = statements.pop() else {panic!("Impossible");};
            self.next_token_match(Token::Rbracket)?;
            eprintln!("[parse_block] exiting block parse");

            self.new_lifetime();
            return Ok(Expr::Block(statements, Box::new(last), Lifetime(self.lifetime)));

        }
        //All Functions return Unit;
        self.next_token_match(Token::Rbracket)?;
        eprintln!("[parse_block] exiting block parse");
        self.new_lifetime();
        return   Ok(Expr::Block(statements, Box::new(Expr::Unit), Lifetime(self.lifetime)));
    }

    fn parse_box(&mut self) -> ParseResult<Expr> {
        eprintln!("[parse_box] parsing box expression");
        self.next_token_match(Token::Box)?;
        self.next_token_match(Token::Lparen)?;
        let e = self.parse_expr()?;
        self.next_token_match(Token::Rparen)?;
        eprintln!("[parse_box] parsed inner expr: {:?}", e);
        Ok(Expr::OBox(Box::new(e)))
    }

    pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        eprintln!("[parse_stmt] entering stmt parse, peek: {:?}", self.peek_token());
        let stmt = match self.peek_token()? {
            Token::Star | Token::Var(_) => {
                eprintln!("[parse_stmt] star found");
                let lv = self.parse_lval()?;
                if *self.peek_token()? == Token::Eq {
                    eprintln!("[parse_stmt] assignment detected");
                    self.next_token()?;
                    let next_expr = self.parse_expr()?;

                    self.next_token_match(Token::Semicolon)?;
                    Stmt::Assign(lv, next_expr)
                } else {
                    self.next_token_match(Token::Semicolon)?;
                    Stmt::Expr(Expr::Lv(lv, Copyable::No))
                }
            }
            Token::Semicolon => {
                eprintln!("[parse_stmt] semicolon only, unit stmt");
                self.next_token()?;
                Stmt::Expr(Expr::Unit)
            }
            Token::Let => {
                eprintln!("[parse_stmt] let found");
                self.next_token()?;
                self.next_token_match(Token::Mut)?;
                let var = self.next_token_var()?;
                self.next_token_match(Token::Eq)?;
                let next_expr = self.parse_expr()?;
                self.next_token_match(Token::Semicolon)?;
                Stmt::LetMut(var, next_expr)
            },

            _ => {
                eprintln!("[parse_stmt] fallback expr stmt");
                let next_expr = self.parse_expr()?;
                self.next_token_match(Token::Semicolon)?;
                Stmt::Expr(next_expr)
            }
        };
        eprintln!("[parse_stmt] exiting stmt parse: {:?}", stmt);
        Ok(stmt)
    }

    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        eprintln!("[parse_expr] entering expr parse, peek: {:?}", self.peek_token());
        let expr = match self.peek_token()? {
            Token::Int(x) => {
                eprintln!("[parse_expr] int literal: {}", x);
                let Token::Int(v) = self.next_token()? else { unreachable!() };
                Expr::Int(v)
            }
            Token::Var(_) | Token::Star => {
                eprintln!("[parse_expr] lval expression");
                let lv = self.parse_lval()?;
                Expr::Lv(lv, Copyable::No)
            }
            Token::Box => {
                return self.parse_box();
            }
            Token::Ampersand => {
                eprintln!("[parse_expr] borrow expression");
                self.next_token()?;
                let mut muta = Mutable::No;
                if *self.peek_token()? == Token::Mut {
                    self.next_token()?;
                    muta = Mutable::Yes;
                }
                let inner_expr = self.parse_lval()?;
                Expr::Borrow(inner_expr, muta)
            }
            Token::Lbracket => {
                return self.parse_block();
            },

            Token::Semicolon => Expr::Unit,

            //You Cannot get a Statement inside of an expression, so these are all the cases there
            //are
            x => return Err(Error::Unexpected(x.clone())) ,
        };
        eprintln!("[parse_expr] exiting expr parse: {:?}", expr);
        Ok(expr)
    }

    pub fn parse_lval(&mut self) -> ParseResult<Lval> {
        eprintln!("[parse_lval] entering lval parse, peek: {:?}", self.peek_token());
        let mut derefs = 0;
        while *self.peek_token()? == Token::Star {
            self.next_token()?;
            derefs += 1;
        }

        let var = self.next_token()?;
        if let Token::Var(x) = var {
            eprintln!("[parse_lval] exiting lval parse: {} with {} derefs", x, derefs);
            Ok(Lval { ident: x, derefs })
        } else {
            eprintln!("Got unexpected in LVAl");
            Err(Error::Unexpected(var))
        }
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        eprintln!("[parse] starting parse");
        self.next_token_match(Token::Fn)?;
        let main = self.next_token()?;
        match main {
            Token::Var(ident) if ident == "main" => {
                eprintln!("[parse] fn main detected");
                self.next_token_match(Token::Lparen)?;
                self.next_token_match(Token::Rparen)?;
                let out = self.parse_block();
                if self.peek_token().is_err() {
                    eprintln!("[parse] completed parse");
                    return out;
                }
                Err(Error::Unexpected(self.next_token()?))
            }
            _ => self.parse_block(),
        }
    }
}
