


use std::collections::HashMap;
use crate::utils::*;


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]

pub struct Slot {
  pub tipe: Type,
  pub lifetime: Lifetime,
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Env(pub HashMap<Ident, Slot>);
impl Env {


 pub fn default() -> Env{
    return Env(HashMap::new());


    }
    pub fn insert(&mut self, var: &str, tipe: Type, lifetime: Lifetime) { 
        let s = Slot{tipe:tipe, lifetime:lifetime};
        self.0.insert(var.to_string(), s);



    }


    pub fn type_lval(&self, lval: &Lval) -> TypeResult<Slot> {
            let  Some(slot) = self.0.get(&lval.ident) else{return Err(Error::UnknownVar(lval.ident.clone()))};

            let mut current_type = &slot.tipe;
            let mut current_lifetime = &slot.lifetime;
            for i in 0..lval.derefs{
                if let Type::Undefined(_) = current_type {
                    return Err(Error::MovedOut(lval.clone()));
                }
                match current_type{
                Type::TBox(x)=> {current_type=&*x;}
                Type::Ref(x,y)=>{
                    
                    let  Some(slot) = self.0.get(&x.ident) else{return Err(Error::UnknownVar(x.ident.clone()))};
                    let Some(tipe) = self.contained(&x.ident)  else{return Err(Error::UnknownVar(x.ident.clone()))};
                     current_type =tipe ;
                     current_lifetime= &slot.lifetime;


                }
                x=>{return Err(Error::CannotDeref(x.clone()));}


                }



        }
        

            let slot = Slot{tipe:current_type.clone(), lifetime:current_lifetime.clone()};
            //self.0.insert(lval.ident.clone(), slot);
            return Ok(slot);

    }

    // Returns the type under the boxes of a type, given that the
    // underlying type is defined
    pub fn contained(&self, var: &Ident) -> Option<&Type> {

        let Some(slot) = self.0.get(var) else {panic!("Impossible")};
        let mut current_type = &slot.tipe;
        loop{

            match current_type{
            Type::TBox(x)=> {current_type=&*x;}
            Type::Undefined(x) =>{return None;} 
            x=>{return Some(&x);},


            }

        }

        return None;

    }


   pub fn immutable(&self, ty1:Type)-> bool{
    match ty1{
        Type::Unit=>{return false;},
        Type::Int=>{return false;},
        Type::TBox(x) =>{self.immutable(*x)},
        Type::Undefined(x) =>{self.immutable(*x)},
        Type::Ref(lv,Mutable::No)=>{

        return true;}

        Type::Ref(lv,Mutable::Yes)=>{
        let tipe = self.type_lval(&lv);


 return self.immutable(tipe.unwrap().tipe);}

        }


    } 


	pub fn contains(&self, ty1:Type,ty2:Type)->bool{
        match ty1 {
        Type::TBox(x) => {self.contains(*x,ty2)}
        x => {
        if x == ty2{
                return true;
            }
        return false;}}}


    pub fn read_prohibited(&self, lval: &Lval) -> bool {
    
    for vars in self.0.keys(){
    let value = self.contained(vars);
    if let Some(Type::Ref(x, Mutable::Yes)) = value{
            if x.ident==lval.ident{

                return true;
                }
            }}



    return false;


    }

    pub fn write_prohibited(&self, lval: &Lval) -> bool {
        if self.read_prohibited(lval){
            return true;
        }


        for vars in self.0.keys(){
            let value = self.contained(vars);
            if let Some(Type::Ref(x, y)) = value{
                if *x.ident==lval.ident{

                    return true;
                    }
                }}

        return false;

    }

    // "move" is a keyword in Rust
    pub fn moove(&mut self, lval: &Lval) -> TypeResult<()> {
        if self.write_prohibited(lval){
        return Err(Error::Dummy);}

        let  Some(current) = self.0.get(&lval.ident) else {return Err(Error::Dummy)};
        let result = self.moove_nested(current.tipe.clone(), lval.derefs.clone());
        if let Ok(tipe) = result{

            let  Some(current) = self.0.get(&lval.ident) else {return Err(Error::Dummy)};
            let slot = Slot{tipe:tipe, lifetime:current.lifetime.clone()};
            self.0.insert(lval.ident.clone(), slot);
            return Ok(())

        }

    return  Err(Error::Dummy);

    }
    
    pub fn moove_nested(&mut self, tipe:Type, i:usize) ->  Result<Type, Error> {
            if i ==0{
            return Ok(Type::Undefined(Box::new(tipe)))}

        match tipe{

            Type::TBox(x) =>{
            let Ok(rest) = self.moove_nested(*x,i-1) else{return Err(Error::Dummy)};
                
            return Ok(Type::TBox(Box::new(rest)))},
            Type::Ref(lval, Mutable::Yes) =>{ return Err(Error::Dummy)},
            Type::Undefined(x)=>{return Err(Error::Dummy)},
            x=>{return Err(Error::Dummy)}


            }


    }
    



    


    // so is "mut"
    pub fn muut(&self, lval: &Lval) -> bool {
    
        let  Some(tipe) = self.contained(&lval.ident) else{return false};

        let mut current_type = tipe;
        let mut i = lval.derefs;
        while i >0{
        
            match current_type{
            Type::Ref(lv, Mutable::Yes) =>{
                    i-=1; 
                    let  Some(tipe) = self.contained(&lv.ident) else {return true;};

                    current_type = tipe;},
            Type::Ref(lv, Mutable::No) =>{return false;},

            Type::TBox(x) =>{
                    current_type= &*x},
            Type::Undefined(x)=>{
                    current_type = &*x},
            _ =>{return true;}


            }

    }
    return true;
    }

    pub fn compatible(&self, t1: &Type, t2: &Type) -> bool { 
    
            match (t1,t2){
            (Type::Int, Type::Int) => {return true;},
            (Type::Unit, Type::Unit) => {return true;},
            (Type::TBox(x), Type::TBox(y))=>{self.compatible(x,y)},
            (Type::Ref(x,Mutable::Yes), Type::Ref(y, Mutable::Yes))=>{return true;},
            (Type::Ref(x,Mutable::No), Type::Ref(y, Mutable::No))=>{return true;},
            (Type::Undefined(x), y) =>{self.compatible(x, y)},
            (y,Type::Undefined(x)) =>{self.compatible(x, y)},
            _=>{return false;}


            }

    }
    
    pub fn update(&mut self, old: Type, new: Type, i: i32) -> Result<Type, Error>{
        if i ==0{
           return Ok(new);
        }


        match old{
        Type::TBox(x) =>{
        let (replaced)= self.update(*x,new,i-1)?;
        return Ok(Type::TBox(Box::new(replaced)))
                },
        Type::Ref(lv, Mutable::Yes) =>{
               self.write(&lv, new.clone())?;
                
                Ok(Type::Ref(lv,Mutable::Yes))
            },

        Type::Ref(lv, Mutable::No) =>{
                return Err(Error::UpdateBehindImmRef(lv.clone()));
             
            },
        Type::Undefined(x)=>{

        let (replaced)= self.update(*x,new,i-1)?;
        return Ok(Type::Undefined(Box::new(replaced)))

            }


        x=>{Ok(new)}}}


    pub fn write(&mut self, w: &Lval, tipe: Type) -> TypeResult<()> {
        //self.0.insert(&w.ident, tipe);
        let current = self.0.remove(&w.ident)
            .ok_or(Error::Dummy)?;
         let (rest) = self.update(current.tipe.clone(), tipe, w.derefs.try_into().unwrap())?;
        
        let slot = Slot{tipe:rest, lifetime:current.lifetime.clone()};
        self.0.insert(w.ident.clone(), slot);
        Ok(())
    }

    pub fn drop(&mut self, l: Lifetime) {
    
        let mut to_drop = Vec::new();
        for value in self.0.keys(){
        let Some(slot) = self.0.get(value) else{panic!("Impossible");};
        if slot.lifetime==l{
        to_drop.push(value.clone());}}

        for value in to_drop{
            self.0.remove(&value);
            
            }


    }
}

 #[derive(PartialEq)]
#[derive(Debug)]
   pub enum Error {
    Dummy,
    UnknownVar(String),
    CannotDeref(Type),
    MovedOut(Lval),
    MoveBehindRef(Lval),
    UpdateBehindImmRef(Lval),
    CopyAfterMutBorrow(Lval),
    MoveAfterBorrow(Lval),
    MutBorrowBehindImmRef(Lval),
    MutBorrowAfterBorrow(Lval),
    BorrowAfterMutBorrow(Lval),
    Shadowing(String),
    IncompatibleTypes(Type, Type),
    LifetimeTooShort(Expr),
    AssignAfterBorrow(Lval),
}

type TypeResult<T> = Result<T, Error>;
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Context {
    pub env: Env,
    // TODO: anything else you need
}
impl Context {
    // l ≥ m, the ordering relation on liftimes (Note (2) pg. 13)
    pub fn lifetime_contains(&self, l: Lifetime, m: Lifetime) -> bool { 
    l<=m

    }
pub fn is_copyable(t: &Type) -> bool {
    match t {
      Type::Int                        => true,
      Type::Ref(_, Mutable::No)       => true,
      _                                => false,
    }
}
    pub fn default() -> Context{
    return Context{env:Env::default()};
}
    // Γ ⊢ T ≥ l (Definition 3.21)
    pub fn well_formed(&self, tipe: &Type, l: Lifetime) -> bool {
        match tipe{
        Type::Unit=>{true},
        Type::Int=>{true},
        Type::TBox(x)=> {self.well_formed(x,l)},
        Type::Ref(lv, Mutable)=>{
        let Some(slot) = self.env.0.get(&lv.ident) else{return false};
        self.lifetime_contains(slot.lifetime.clone(), l)}
        Type::Undefined(x)=>{self.well_formed(x,l)}
        
            }


    }
    

    pub fn type_expr(&mut self, expr: &mut Expr) -> TypeResult<Type> { 
    match expr{
      Expr::Unit => Ok(Type::Unit),
      Expr::Int(_) => Ok(Type::Int),
      Expr::Lv(lv, Copyable::Yes) => {

            let slot = self.env.type_lval(lv)?;
            Ok(slot.tipe)
        }

        Expr::Lv(lv, Copyable::No) => {


            let slot = self.env.type_lval(lv)?;  
            let ty   = slot.tipe.clone();

                let contained = self.env.contained(&lv.ident);

             if None== contained {
                    return Err(Error::MovedOut(lv.clone()));
                }
           
            if let Some(Type::Ref(x, Mutable::No))= contained{

                return Err(Error::MoveBehindRef(lv.clone()));}
            if Self::is_copyable(&ty) {
                if self.env.read_prohibited(lv) {
                    return Err(Error::CopyAfterMutBorrow(lv.clone()));
                }
                // turn this load into the “we’ve now copied it” form
                *expr = Expr::Lv(lv.clone(), Copyable::Yes);
                return Ok(ty);
            }

  
            if self.env.read_prohibited(lv) {
                return Err(Error::MoveAfterBorrow(lv.clone()));
            }

            if self.env.write_prohibited(lv) {
               return Err(Error::MoveAfterBorrow(lv.clone()));
            }
            self.env.moove(lv)?;   

            Ok(ty)


        }
        Expr::Borrow(lv, Mutable::Yes) => {

            let slot = self.env.type_lval(lv)?;

            let ty   = slot.tipe.clone();

            let contained = self.env.contained(&lv.ident);
            


        
            if self.env.write_prohibited(lv) {
                return Err(Error::MutBorrowAfterBorrow(lv.clone()));
            }
            if None== contained {
                    return Err(Error::MovedOut(lv.clone())); }
           

            if !self.env.muut(lv) {

                return Err(Error::MutBorrowBehindImmRef(lv.clone()));}

                Ok(Type::Ref(lv.clone(), Mutable::Yes))


        }
        Expr::Borrow(lv, Mutable::No) => {



            let slot = self.env.type_lval(lv)?;
            let contained = self.env.contained(&lv.ident);

            let ty   = slot.tipe.clone();

            if self.env.read_prohibited(lv) {
                return Err(Error::BorrowAfterMutBorrow(lv.clone()));
            }
            

             if None== contained {
                    return Err(Error::MovedOut(lv.clone()));
                }
           

            Ok(Type::Ref(lv.clone(), Mutable::No))
        }
        Expr::OBox(e) => {


            let inner_type = self.type_expr(e)?;
            Ok(Type::TBox(Box::new(inner_type)))
        }

        Expr::Block(stmts, result_expr, lifetime) => {
            for stmt in stmts {


                self.type_stmt(stmt)?;
                if let Stmt::LetMut(x, _) = stmt{
                        let Some(ty_of) = self.env.0.get(x) else {panic!("Impossible");};;
                        let slot = Slot{tipe:ty_of.tipe.clone(), lifetime:lifetime.clone()};
                        self.env.0.insert(x.to_string(), slot);}

                if let Stmt::Assign(lv, expr) = stmt{
                    let slot = self.env.type_lval(&lv)?;
                    if slot.lifetime < *lifetime{
                    return Err(Error::LifetimeTooShort(expr.clone()));
                    }
                    }

                }
            let t = self.type_expr(result_expr)?;
            

            self.env.drop(lifetime.clone());
            Ok(t)
        }
    }
}


    pub fn type_stmt(&mut self, stmt: &mut Stmt) -> TypeResult<()> { 

        match stmt {
        Stmt::Assign(lv, expr) =>{


 
        let old_type = self.env.type_lval(lv)?;
        println!("{:?}",old_type);
        
        let new_type = self.type_expr(expr)?;
        
                

        if !self.env.compatible(&new_type,&old_type.tipe){
            return Err(Error::IncompatibleTypes(old_type.tipe, new_type)); }

        else if self.env.write_prohibited(lv){
            return Err(Error::AssignAfterBorrow(lv.clone()));}
    
                
        else if !self.env.muut(lv){

            return Err(Error::UpdateBehindImmRef(lv.clone())); }
        self.env.write(lv, new_type)?;

        Ok(())

            },


        Stmt::LetMut(x, expr) =>{
        if self.env.0.contains_key(x) {
            return Err(Error::Shadowing(x.to_string()));}

        

        let new_type = self.type_expr(expr)?;
        self.env.insert(x,new_type, Lifetime::global());

        Ok(())
            },
        Stmt::Expr(expr) =>{
            self.type_expr(expr)?;
            Ok(())
            }



        }



    }
}

