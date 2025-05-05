

use std::str::Lines;


    // TODO

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Eq,
    Ampersand,
    Star,
    Comma,
    Semicolon,
    Fn,
    Let,
    Mut,
    Box,
    Int(i32),
    Var(String),
}
pub struct Lexer<'a> {
    contents: Lines<'a>,
    curr_line_num: usize,
    curr_col_num: usize,
    curr_line: &'a str,
}

const LEXEMES : [(&str, Token); 13] = [
    ("(", Token::Lparen),
    (")", Token::Rparen),
    ("{", Token::Lbracket),
    ("}", Token::Rbracket),
    ("=", Token::Eq),
    ("&", Token::Ampersand),
    ("*", Token::Star),
    (",", Token::Comma),
    (";", Token::Semicolon),
    ("fn", Token::Fn),
    ("let", Token::Let),
    ("mut", Token::Mut),
    ("Box::new", Token::Box),
];

#[derive(Debug)]
pub enum Error {
    Unknown(usize, usize),
}

type LexResult = Result<Token, Error>;
impl<'a> Lexer<'a> {

    pub fn new(contents:& 'a str) -> Lexer<'a>{
    return Lexer{contents:contents.lines(), curr_line_num:0, curr_col_num:0, curr_line:""};


    } 


    fn unknown(&self) -> Error {
        Error::Unknown(self.curr_line_num, self.curr_col_num)
    }

    fn consume(&mut self, i: usize) {
    self.curr_line = &self.curr_line[i..];
       self.curr_col_num+=i;}

    fn symbol_or_keyword(&mut self) -> LexResult { 

        
        for (lexeme, token) in LEXEMES {
            if self.curr_line.starts_with(lexeme) {
                self.consume(lexeme.len());
                return Ok(token);
            }
        }
        Err(self.unknown())
    
     }
    // similar to `symbol_or_keyword` but for variables
    fn variable(&mut self) -> LexResult {
        let mut len =0;
        for character in self.curr_line.chars(){
                if character.is_ascii_alphanumeric() || character == '_' {
                    len += 1;
                } else {
                    break;
                }
            }
        if len ==0{
                 return Err(self.unknown());
             }
        let var = &self.curr_line[..len];
        self.consume(len);
        return Ok(Token::Var(var.to_string()));

     }

    fn skip_whitespace(&mut self){
        loop{
            if self.curr_line.is_empty() || self.curr_line.starts_with("//"){
                let Some(temp) = self.contents.next() else {return};

                self.curr_line= temp;
                self.curr_line_num += 1;
                self.curr_col_num = 1;
                continue;
            } 

            let Some(ch) = self.curr_line.chars().next() else{return;};

            if ch.is_whitespace(){
            self.consume(1);}
            else {
                break;
            }


             }
     }

    // similar to `symbol_or_keyword` for but integer literals
    fn int(&mut self) -> LexResult { 
        let mut len =0;
            for character in self.curr_line.chars(){
                if character.is_ascii_digit(){
                         len+=1;
                     }
                else{
                         break;
                     }

                 }
            if len ==0{
                     return Err(self.unknown());
                 }
            let var = &self.curr_line[..len];
            let v =  var.parse::<i32>().unwrap();
             self.consume(len);
            return Ok(Token::Int(v));

             }





     
}
impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;
    fn next(&mut self) -> Option<LexResult> { 

        self.skip_whitespace();

        if self.curr_line.is_empty()  {return None; }
        if let Some((lexeme, token)) = LEXEMES
            .iter()
            .find(|(lex, _)| self.curr_line.starts_with(*lex))
        {
            self.consume(lexeme.len());
            return Some(Ok(token.clone()));
        }


        //Get the very Next character
        let ch = self.curr_line.chars().next().unwrap();
        //If it is a digit return int
        let tok = if ch.is_ascii_digit() {
            self.int()

        //IF it is alphanumeric return a vairable
        } else if ch.is_ascii_alphabetic() || ch == '_' {
            self.variable()


         }
        //Otherwise just return a symbol
         else {
            self.symbol_or_keyword()
        };
        Some(tok)


     }
}

