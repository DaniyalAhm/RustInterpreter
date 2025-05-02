use crate::lexer::Lexer;
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    // TODO: anything else you need
}

pub enum Error {
    EndOfFile,
    Lexer(lexer::Error),
    Unexpected(lexer::Token),
}

type ParseResult<T> = Result<T, Error>;


impl<'a> Parser<'a> {
    fn next_token(&mut self) -> ParseResult<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(err)) => Err(Error::Lexer(err)),
            None => Err(Error::EndOfFile),
        }
    }

    fn next_token_match(&mut self, t: Token) -> ParseResult<Token> {
        let token = self.next_token()?;
        if token != t {
            Err(Error::Unexpected(token))
        } else {
            Ok(token)
        }
    }

    fn next_token_var(&mut self) -> ParseResult<String> {
        let token = self.next_token()?;
        match token {
            Token::Var(ident) => Ok(ident.clone()),
            _ => Err(Error::Unexpected(token))
        }
    }

    fn peek_token(&mut self) -> Result<&Token, Error> {
        let token = self.lexer.peek();
        match token {
            Some(Ok(token)) => Ok(token),
            _ => Err(Error::EndOfFile),
        }
    }
}


impl<'a> Parser<'a> {
    fn parse_block(&mut self) -> ParseResult<Expr> {
    self.next_token_match(Token::Box)?;
    self.next_token_match(Token::Lparen)?;
    let e = self.parse_expr()?;
    self.next_token_match(Token::Rparen)?;
    Ok(Expr::Box(Box::new(e)))



    }

    fn parse(&mut self) -> ParseResult<Expr> {
        self.next_token_match(Token::Fn)?;
        let main = self.next_token()?;
        match main {
            Token::Var(ident) if ident == "main" => {
                self.next_token_match(Token::Lparen)?;
                self.next_token_match(Token::Rparen)?;
                let out = self.parse_block();
                if self.peek_token().is_err() {
                    return out
                }
                Err(Error::Unexpected(self.next_token()?))
            }
            _ => Err(Error::Unexpected(main))
        }
    }
}
