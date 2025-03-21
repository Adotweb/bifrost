use crate::*;

#[derive(Clone)]
pub enum Expression {

    LiteralStr(String),
    LiteralNum(f64),
    LiteralBool(bool),
    LiteralID(String),
    LiteralNil,

    Binary{
        left : Box<Expression>,
        operator : Token,
        right : Box<Expression>
    },

    Unary{
        operator : Token,
        right : Box<Expression>
    },

    Grp{
        inner : Box<Expression>
    },
    
    Block{
        expressions : Vec<Expression>
    },

    If{
        condition : Box<Expression>,
        if_block : Box<Expression>,
        //as many as you wish
        else_if_block : Option<Vec<Box<Expression>>>,
        else_block : Option<Box<Expression>>
    },

    While{
        condition : Box<Expression>,
        block : Box<Expression>
    },

    For{
        condition : Box<Expression>,
        block : Box<Expression>
    },

    Fn{
        arguments : Vec<Token>,
        body : Box<Expression>
    },

    Assign{
        target : Box<Expression>,
        value : Box<Expression>,
    },

    Declaration{
        name : Token,
        value : Box<Expression>,
        constant : bool
    }
}

impl Expression{
    pub fn expr(&self) -> Result<Expression, Error>{
        return Ok(self.clone())
    }
}

pub type FallibleExpression = Result<Expression, Error>;


fn get_current_token(tokens : &Vec<Token>, current_index : &mut usize) -> Result<Token, Error>{
    if let Some(token) = tokens.get(*current_index){
        Ok(token.clone())
    } else {
        Err(Error::Nil)
    }
}

//checks if there is a next token, and returns it as well as consuming the current token
fn consume_token(tokens : &Vec<Token>, current_index : &mut usize) -> Result<Token, Error>{
    if let Some(token) = tokens.get(*current_index + 1){
        *current_index += 1;
        Ok(token.clone())
    } else {
        Err(Error::Nil)
    }
}

//does the same as consume_token but checks first if the current token has some type
fn match_token(tokens : &Vec<Token>, current_index : &mut usize, check_token : TokenType) -> Result<Token, Error>{

    if let Some(next_token) = tokens.get(*current_index + 1){
        let token = get_current_token(tokens, current_index)?;
        if token.check_against_token_type(check_token){
           
            *current_index += 1;
            Ok(next_token.clone())

        } else {
            Err(Error::Nil)
        }

    } else {
        Err(Error::Nil)
    }

}

//instead of checking moving and and throwing, this function returns a bool *IF* the current token
//matches
fn match_tokens(tokens : &Vec<Token>, current_index : &mut usize, check_tokens : Vec<TokenType>) -> Result<bool, Error>{

    if let Some(next_token) = tokens.get(*current_index + 1){
        let token = get_current_token(tokens, current_index)?;

        if check_tokens.iter().map(|x| token.check_against_token_type(x.clone())).any(|x| x == true){
           
            *current_index += 1;
            Ok(true)

        } else {
            return Ok(false)
        }

    } else {
        Err(Error::Nil)
    }

}


fn expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    
}

fn term(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    
    let left = factor(tokens, current_index)?;

    if 
    
}

fn factor(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{


}

fn unary(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{


}

fn primary(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
   
    let token = get_current_token(tokens, current_index)?;
    match token.r#type{
        
        TokenType::ID(name) => Expression::LiteralID(name).expr(),
        TokenType::NUM(number) => Expression::LiteralNum(number.parse::<f64>().unwrap()).expr(),
        TokenType::STR(string) => Expression::LiteralStr(string).expr(),

        TokenType::LPAREN => {
            consume_token(tokens, current_index)?;
            expr(tokens, current_index)
        },
    
        

        _ => Err(Error::Nil) 
    }

}

pub fn parse(tokens : Vec<Token>) -> Result<Vec<Expression>, &'static str> {

    let mut expressions = Vec::new();
    let mut index = 0;

    while let Some(token) = tokens.get(index){
         
        
    }


    Ok(expressions)
}
