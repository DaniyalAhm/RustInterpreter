use crate::enumVals::Reference;
use crate::Terms::LetMUT;

use crate::Type::immutabe_borrow;
use crate::LVals::Deference;
 use crate::enumRef::NotOwned;
use crate::Terms::value;
use crate::Terms::copy;
use crate::Terms::r#move;
use crate::Terms::mutBorrow;
use std::collections::HashMap;
use crate::Terms::Block;
use crate::LVals::Var;
use std::collections::HashSet;
use crate::Type::mut_borrow;
use crate::PartialTypes::ty;
 use crate::PartialTypes::partial_box;


struct Expr{T:Box<Terms>}


#[derive(Clone)]
struct slot {term:enumVals, lifetime:Lifetime}
struct Context {store:HashMap<String, slot>} 
struct Lifetime(usize);
struct Lval {
    ident: Ident,
    derefs: usize,
}

struct Slot {
    value: Pvalue,
    lifetime: Lifetime,

}
type Pvalue = Option<Value>;
type Ident = String;


enum Value {
    Unit,
    Int(i32),
    Ref(Location, Owned),
}

//NOTE: First lets define a Enum for Structs
//NOTE: These are all the Enums we will need for semantics based of FR Spec


enum Copyable {Yes, No}
enum Mutable {Yes, No}


enum Expr {
    Unit,
    Int(i32),
    Lval(Lval, Copyable),
    Box(Box<Expr>),
    Borrow(Lval, Mutable),
    Block(Vec<Stmt>, Box<Expr>, Lifetime),
}

enum Stmt {
    Assign(Lval, Expr),
    LetMut(Ident, Expr),
    Expr(Expr),
}

Location = Ident;
enum Owned {Yes, No}




struct Store(HashMap<Location, Slot>)
impl Store {
    fn locate(&self, w: &Lval) -> &Location {
    return w.ident; }
    fn read(&self, x: &Lval) -> &Slot {
    let Some(x_slot)= self.get(x) else{panic!("Impossible")}     
    return x_slot;}
    fn write(&mut self, x: &Lval, v: Pvalue) -> Pvalue { 
    todo!()

    }
    fn drop(&mut self, values: Vec<Pvalue>) { todo! () }
}

struct Context {
    store: Store,
    // TODO: anything else you need
}




impl Context {
  fn eval_expr(&mut self, e : &Expr, l: Lifetime) -> Value 
    {



    match (e).as_ref() {
        Block(s)=>{todo!()},

        LetMUT(x,ty_e, y)=>{

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


//NOTE: Type Checking Part!


//NOTE: Luckily I Always Write PseudoCode Before any task just gotta convert
//
enum Error{
    testing

}


// TODO




//NOTE: Making my Own Enums and What not
//

#[derive(PartialEq)]
enum PartialTypes{
	partial_box(Box<PartialTypes>),
	Undefined,
	ty(Type)
}
#[derive(Clone)]
#[derive(PartialEq)]
enum Type {
    Unit,
    Int,
    Box(Box<Type>),
    Ref(Lval, Mutable),
    Undefined(Box<Type>),
}



struct Gamma 
{context:HashMap<LVals,Type>,
all_types:HashSet<LVals>,
path_conflicts:HashMap<(Expr,Expr),bool>
}








impl Gamma{
	fn contains(&self, ty1:PartialTypes,ty2:PartialTypes)->bool{
	match ty1 {
	partial_box(x) => {self.contains(*x,ty2)}
	x => {
    if x == ty2{
            return true;
        }
    return false;}


	


}}


	fn findSink(&self,x:LVals) -> LVals {
	match (x) {
	Deference(x) =>(self.findSink(*x)),
	(x)=>x
	
	}}

	fn PathConflict(&self,x:LVals, y:LVals) -> bool{
	let x_sink = self.findSink(x);
	let y_sink= self.findSink(y);
	if x_sink == y_sink {return true;}
	return false;

}


	fn ReadProhibited(&self, val:LVals)-> bool{
    let x_sink = self.findSink(val.clone());
    let Some(x_type) = self.context.get(&x_sink) else{ return false};
    if let mut_borrow(y) = x_type{return true;}

    for variable in self.context.keys(){
        if self.PathConflict(variable.clone(), val.clone()){
                return true;
            }


        }	
	return false;}

    fn copy(&self,val:LVals)-> LVals{
       return val.clone(); 



    }


    // "move" is a keyword in Rust
    fn moove(&mut self, lval: &Lval) -> TypeResult<()> { todo!() }

    // so is "mut"
    fn muut(&self, lval: &Lval) -> bool { 
    


    }

    fn compatible(&self, t1: &Type, t2: &Type) -> bool { todo!() }

    fn write(&mut self, w: &Lval, tipe: Type) -> TypeResult<()> { todo!() }

    fn drop(&mut self, l: Lifetime) { todo!() }
    

    }



/*
    fn WriteProhibited(&self, val:LVals) -> bool{
        
    let x_sink = self.findSink(val.clone());
    let Some(x_type) = self.context.get(&x_sink) else{ return false};
    if let immutabe_borrow(x)=x_type{return true;}


        if self.ReadProhibited(val) {
        return true;
        
        }

        return false;
        }


        fn type_expr(&mut self, expr: &mut Expr) -> Result<Type, Error> {
        match  (expr.T).as_ref(){
        Terms::Block(s)=>{todo!()},
        Terms::LetMUT(x,ty_e,e1)=>{
        self.context.insert(LVals::Var(x.clone()),ty_e.clone());
        return Ok(ty_e.clone()); 

                ;},
        Terms::mutBorrow(s)=>{todo!()
        if self.WriteProhibited(s){
                //Return Error
                todo!()}
        
        return Ok(Type::mut_borrow(s))
    

            },
        Terms::r#move(lv)=>{todo!()},

        Terms::copy(lv)=>{
        if self.ReadProhibited(lv.clone()){
         //Make Errors
        todo!()}
        let lv2 = self.copy(lv.clone());
        let lv2_sink = self.findSink(lv2);
        let Some(ty_e) = self.context.get(&lv2_sink) else{todo!()};
        return Ok(ty_e.clone());},
        Terms::value(Unit)=>{return Ok(Type::Unit)}
        Terms::value(Int)=>{return Ok(Type::Int)}

        Terms::value(ref_ty,lv)=>{
        todo!()}
        }




    }


*/





}
