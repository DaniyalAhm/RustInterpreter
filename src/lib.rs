use crate::enumVals::Reference;
use crate::Terms::LetMUT;

use crate::LVals::Deference;
 use crate::enumRef::NotOwned;
use crate::Terms::value;
use crate::Terms::copy;
use crate::Terms::r#move;
use crate::Terms::mutBorrow;
use std::collections::HashMap;
use crate::Terms::Block;
use crate::LVals::Var;



struct Expr{T:Box<Terms>}

#[derive(Clone)]
struct Value{Val:enumVals}
#[derive(Clone)]
struct Lifetime{m:String}

#[derive(Clone)]
struct slot {term:enumVals, lifetime:Lifetime}
struct Context {store:HashMap<String, slot>} 




//NOTE: First lets define a Enum for Structs
//NOTE: These are all the Enums we will need for semantics based of FR Spec


#[derive(Clone)]
enum Terms{
    Block(slot), 
    LetMUT(String, Box<Terms>),
    mutBorrow(LVals),
    r#move(LVals),
    copy(LVals),
    value(enumVals),
}

#[derive(Clone)]
enum LVals{
Var(String),
Deference(Box<LVals>)
}

#[derive(Clone)]
enum enumRef{
NotOwned,
Owned
}
#[derive(Clone)]
enum enumVals{
    Unit,
    Integer(i32),
    Reference(enumRef, LVals)

}

fn unwrap(val: LVals) ->String {
    let Var(x) = val else{return unwrap(val)};
    return x;}

fn unwrap_once(val: LVals) ->LVals {
    match val{
    Var(x) => {Var(x)}
    Deference(x) => {*x}

    }
}

impl Context {
  fn eval_expr(&mut self, e : &Expr, l: Lifetime) -> Value 
    {



    match (e.T).as_ref() {
        Block(s)=>{todo!()},

        LetMUT(x, y)=>{

            let new_term = Expr{T:Box::new(*y.clone())};
            let value_of_y = self.eval_expr(&new_term,l.clone());
            let new_slot= slot{term:value_of_y.Val.clone(),lifetime:l.clone()};

            self.store.insert(x.to_string(),new_slot);
            return value_of_y }
        mutBorrow(s)=>{
        
        let unwrapped = unwrap_once(s.clone());



    
        let value_of_e= enumVals::Reference(enumRef::NotOwned, unwrapped);
        return Value { Val: value_of_e };

            },
        r#move(s)=>{
        let w = unwrap(s.clone());
        let Some(val) = self.store.remove(&w) else{panic!("Impossible")};
        return Value{Val: val.term};


            },
        copy(s)=>{
        let w = unwrap(s.clone()); 
        let Some(val) = self.store.get(&w) else{panic!("Impossible")};
        return Value{Val: val.term.clone()};

        },
        value(val)=>{

        return Value{Val: val.clone()};
           },
        }    
    }
}

