use crate::{Expression, Error, Type, Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;


pub struct TypeEnvironment {
    //this is where assignments go to, be it consts or lets, it does not matter since reassignment
    //is not possible across types (except for any)
    values : HashMap<String, Type>,

    //this is where stuff starting with type something goes to
    types : HashMap<String, Type>,
    //consts cannot be changed

    enclosing : Option<Rc<RefCell<TypeEnvironment>>>,
    //this shows all possible mappings inside the current environment, because yes, even
    //overloadings are not global,
    //we always have this : (Operation, Type, Type) -> Type, that shows the operation that takes
    //type x type -> type
    overloadings : HashMap<(Token, Type, Type), Type>
}

impl TypeEnvironment{
    //returns a new environemnt that is enclosed in the old one (is "nested" inside)
    pub fn enclose(&mut self, enclosing : TypeEnvironment) -> TypeEnvironment{  
        
        Self{
            enclosing : Some(Rc::new(RefCell::new(enclosing))),
            ..Default::default()
        } 
    } 

    pub fn new() -> Self{
        return Self{
            ..Default::default()
        }
    }


    pub fn get_type(&mut self, key : String) -> Option<Type>{
       
        //check in the current environment before moving upwards
        if let Some(type_inside) = self.values.get(&key){
            return Some(type_inside.clone())
        } 

        //then we check upwards (recursively)
        if let Some(enclosing) = &self.enclosing{
            return enclosing.borrow_mut().get_type(key);
        }


        //if all this fails we return none, meaning the type does not exist in the codebase
        return None
    } 
   

    pub fn assign_type(&mut self, key : String, assign_type : Type) -> Result<(), Error>{

        //check if the value already exists and if yes we return (since we cannot set)
        if let Some(_) = self.values.get(&key){
            return Ok(());
        }

        self.values.insert(key, assign_type);
         
        Ok(())
    }

    //checks the type provided against the keyed type inside the environment
    pub fn check_type(&mut self, key : String, check_type : Type) -> Result<bool, Error> {
        if let Some(type_inside) = self.values.get(&key)  {
            return Ok(*type_inside == check_type)
        }
        return Err(Error::TypeNotFound)
    }
}

impl Default for TypeEnvironment{
    fn default() -> Self {
        Self{
            enclosing : None,
            overloadings : HashMap::new(),
            types : HashMap::new(),
            values : HashMap::new()
        }
    }
}

pub fn check_types(ast : Vec<Expression>) -> Result<(), Error>{

    let global_env : TypeEnvironment = TypeEnvironment::new();


    for expression in ast{
        
    } 


    
    Ok(())
} 
