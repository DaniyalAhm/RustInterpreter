

//LIFE TIME IMPLEMENTATION
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Lifetime(pub usize);


impl Lifetime {
    pub fn global() -> Lifetime {
        Lifetime(0)
    }
}

/// LVAL DEFINATIONS
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Hash)]

#[derive(Debug)]
#[derive(Eq)]
pub struct Lval {
    pub ident: Ident,
    pub derefs: usize,
}
impl Lval{
   pub fn new(id:  &str, der:usize) -> Lval{
        return Lval{ident:id.to_string(), derefs:der};

    }
    pub fn var(id: &str) -> Lval{
    return Lval{ident:id.to_string(), derefs:0};


    }}

///NOTE: Basic Types 
pub type Pvalue = Option<Value>;
pub type Ident = String;

pub type Location = Ident;

/// Basic ENUMS NEEDED 
///


#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Copyable {Yes, No}



#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Mutable {Yes, No}



#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Owned {Yes, No}


/* FOR REFERENCE
*
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

* */


// EXPRESSIONS DEFINATION

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {
    Unit,
    Int(i32),
    Lv(Lval, Copyable),
    OBox(Box<Expr>),
    Borrow(Lval, Mutable),
    Block(Vec<Stmt>, Box<Expr>, Lifetime),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Stmt {
    Assign(Lval, Expr),
    LetMut(Ident, Expr),
    Expr(Expr),
}


//VALUE DEFINATION

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Value {
    Unit,
    Int(i32),
    Ref(Location, Owned),
}




//NOTE: Type DEFINATIONs


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Type {
    Unit,
    Int,
    TBox(Box<Type>),
    Ref(Lval, Mutable),
    Undefined(Box<Type>),
}

