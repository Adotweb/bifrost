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
}


pub fn check_types(ast : Vec<Expression>) -> Result<(), Expression>{

    let global_env : TypeEnvironment = TypeEnvironment::new();


    for expression in ast{
         
    } 


    
    Ok(())
} 
