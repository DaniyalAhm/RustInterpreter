
use std::collections::HashMap;
use crate::Value::Int;
use crate::Value::Ref;
use crate::Expr;
use crate::Lval;
use crate::Expr::Block;
use crate::Expr::Borrow;
use crate::Expr::OBox;

use crate::Stmt;



#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Lifetime(usize);


impl Lifetime {
    pub fn global() -> Lifetime {
        Lifetime(0)
    }
}
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Hash)]

#[derive(Debug)]
#[derive(Eq)]
struct Lval {
    ident: Ident,
    derefs: usize,
}
impl Lval{
    fn new(id:  &str, der:usize) -> Lval{
        return Lval{ident:id.to_string(), derefs:der};

    }
    fn var(id: &str) -> Lval{
    return Lval{ident:id.to_string(), derefs:0};


    }


}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Slot {
    value: Pvalue,
    lifetime: Lifetime,

}

type Pvalue = Option<Value>;
type Ident = String;

type Location = Ident;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Value {
    Unit,
    Int(i32),
    Ref(Location, Owned),
}

//NOTE: First lets define a Enum for Structs
//NOTE: These are all the Enums we will need for semantics based of FR Spec

#[derive(Debug)]
#[derive(PartialEq)]
enum Copyable {Yes, No}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Mutable {Yes, No}

#[derive(Debug)]
#[derive(PartialEq)]
enum Expr {
    Unit,
    Int(i32),
    Lv(Lval, Copyable),
    OBox(Box<Expr>),
    Borrow(Lval, Mutable),
    Block(Vec<Stmt>, Box<Expr>, Lifetime),
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Stmt {
    Assign(Lval, Expr),
    LetMut(Ident, Expr),
    Expr(Expr),
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Owned {Yes, No}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Store(HashMap<Location, Slot>);


impl Store {
 pub fn default() -> Store{
    return Store(HashMap::new());


    }
fn locs_by_lifetime(& self, l:Lifetime) -> Vec<Pvalue>{
    let mut result = Vec::new();
    for locs in self.0.keys(){
    let Some(s) = self.0.get(locs) else {panic!("Impossible")};
    if s.lifetime == l{
                result.push(Some(Value::Ref(locs.to_string(),Owned::Yes)));
                


            }


        }
    println!("{:?}", result); 
    return result;


    }
    
fn insert(&mut self, loc:&str, val:Pvalue, l:Lifetime){


    let slot = Slot{value:val, lifetime:l};
    self.0.insert(loc.to_string(),slot);


    }

    ///SLIGHT DEFINATION CHANGE
 fn locate<'a>(&'a self, w: &'a Lval) -> &'a Location {
    let mut current_def = &w.ident;
    for i in 0..w.derefs{

        let Some(slot) = self.0.get(current_def) else {panic!("Impossible")};
        match &slot.value{
        Some(Value::Unit) => {panic!("Impossible");;},
        Some(Value::Int(x)) => {panic!("Impossible");;},
        Some(Value::Ref(loc,owned)) =>{current_def = loc;},
        None =>{panic!("Impossible");}


        }

        }
    return current_def;

 }


fn read(&self, x: &Lval) -> &Slot {


    let loc = self.locate(x);    
    return self.0.get(loc).expect("Impossible");}


fn write(&mut self, x: &Lval, v: Pvalue) -> Pvalue { 
    //First we Must Create the Life time.
    //We can set the Location as the Variable itself

    let loc = self.locate(x).clone();
    let Some(old_slot)= self.0.remove(&loc) else {panic!("Impossible");};
    let s=  Slot{value: v.clone(), lifetime:old_slot.lifetime.clone()};
    self.0.insert(loc.to_string(),s);


    return old_slot.value.clone();

    }
    fn drop(&mut self, values: Vec<Pvalue>) { 
        let mut result = Vec::new(); 
        for value in values{
            match value {

            Some(Value::Ref(loc,Owned::Yes)) =>{
            let Some(next) = self.0.remove(&loc) else {panic!("Impossible")};;
                result.push(next.value);}

            Some(Value::Ref(loc,Owned::No)) =>{},


            Some(literal) =>{},
            None =>{}}}
        if result.len() !=0{
        self.drop(result);

            }

    }

}



struct Context {
    store: Store,
    // TODO: anything else you need
    num:i32,



}



impl Context {
    fn fresh (&mut self) -> Location{
    self.num+=1;
        return "x".to_owned()+&self.num.to_string();


    }
    fn default() -> Context{
    return Context{store:Store::default(), num:0};



    }
    pub fn eval_expr(&mut self, expr: &Expr, l: Lifetime) -> Value {
    match expr{

    Expr::Unit =>{return Value::Unit},
    Expr::Int(x)=>{return Value::Int(x.clone())}
    Expr::Lv(x, Copyable::Yes)=>{

        let loc = self.store.locate(x);
        let Some( x_slot) = self.store.0.get(loc) else{panic!("Impossible");};
        let Some(ref x_val) = x_slot.value else {panic!("Impossible");};


        return x_val.clone(); }


    Expr::Lv(x, Copyable::No)=>{

        let old_val = self.store.write(x, None)
        .expect("move out of a non-existent slot");

        return old_val; }


    Expr:: OBox(e1)=>{
        let val_e1 = self.eval_expr(e1,l.clone());
        let s = Slot{value:Some(val_e1.clone()), lifetime:Lifetime::global()};
        let loc = self.fresh();
        self.store.0.insert(loc.clone(), s);
        
        


    return Value::Ref(loc,Owned::Yes);

            }
    Expr::Borrow(x, mutability)=>{
        let place = self.store.locate(x);
        return Value::Ref(place.clone(), Owned::No);





            }
    Expr::Block(statements, e1, l1)=>{
        for expr_st in statements{
            self.eval_stmt(expr_st, l1.clone());}

        let value = self.eval_expr(e1,l1.clone());
        self.store.drop(self.store.locs_by_lifetime(l1.clone()));
        return value; 



            }



        }



    }
    pub fn eval_stmt(&mut self, stmt: &Stmt, l: Lifetime) {
        match stmt {
        Stmt::Assign(lv, expr) =>{
        let (val_expr) = self.eval_expr(expr,l);

        let old_val= self.store.read(lv);

        self.store.drop([old_val.value.clone()].to_vec());
        self.store.write(lv, Some(val_expr));


            },


        Stmt::LetMut(x, expr) =>{
        
        let (val_expr) = self.eval_expr(expr,l.clone());
        let s = Slot{value:Some(val_expr.clone()), lifetime:l.clone()};
                self.store.0.insert(x.to_string(),s);


            },
        Stmt::Expr(expr) =>{
                self.eval_expr(expr,l);
            }



        }





    }
}
#[cfg(test)]
mod tests {
    mod tests1;
}

//NOTE: Type Checking Part!


//NOTE: Luckily I Always Write PseudoCode Before any task just gotta convert
//
enum Error{
    testing

}


// TODO




//NOTE: Making my Own Enums and What not
//
/*
#[derive(PartialEq)]
enum PartialTypes{
	partial_box(Box<PartialTypes>),
	Undefined,
	ty(Type)
}
*/
/*
#[derive(Clone)]
#[derive(PartialEq)]
enum Type {
    Unit,
    Int,
    Box(Box<Type>),
    Ref(Lval, Mutable),
    Undefined(Box<Type>),
}
*/
