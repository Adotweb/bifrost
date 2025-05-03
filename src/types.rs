use crate::{Expression, Error, Type};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct TypeEnvironment {
    values : HashMap<String, Type>,
    enclosing : Option<Arc<Mutex<TypeEnvironment>>>
}

impl TypeEnvironment{
    //returns a new environemnt that is enclosed in the old one (is "nested" inside)
    pub fn enclose(&mut self, enclosing : TypeEnvironment) -> TypeEnvironment{  
        Self{
            values : HashMap::new(),
            enclosing : Some(Arc::new(Mutex::new(enclosing)))
        } 
    } 

    pub fn new() -> Self{
        return Self{
            values : HashMap::new(),
            enclosing : None
        } 
    }

    //this puts a new type into the current environment
    pub fn assign_type(&mut self, key : String, assign_type : Type) -> Result<(), Error>{
        self.values.insert(key, assign_type);
        Ok(())
    }
   

    //this tries to do the same, but instead checks if the key exists already and checks if the
    //things are compatible
    pub fn reassign_type(&mut self, key : String, assign_type : Type) -> Result<(), Error>{
        self.values.insert(key, assign_type);
        Ok(())
    }
}


pub fn check_types(ast : Vec<Expression>) -> Result<(), Error>{

    let global_env : TypeEnvironment = TypeEnvironment::new();


    for expression in ast{
         
    } 


    
    Ok(())
} 
