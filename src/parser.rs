use crate::*;

#[derive(Clone, Debug, PartialEq)]
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

    Return {
        value : Box<Expression>
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

    FunctionCall{
        function : Box<Expression>,
        arguments: Vec<Expression>
    },

    FieldCall{
        target : Box<Expression>,
        value : Box<Expression>
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
        if token.check_against_token_type(check_token.clone()){
           
            *current_index += 1;
            Ok(next_token.clone())

        } else {
            Err(Error::UnexpectedToken{
                expected : check_token,
                unexpected : token.r#type
            })
        }

    } else {
        Err(Error::UnexpectedToken{
            expected : check_token,
            unexpected : TokenType::EOF
        })
    }

}

//instead of checking moving and and throwing, this function returns a bool *IF* the current token
//matches
fn match_tokens(tokens : &Vec<Token>, current_index : &mut usize, check_tokens : Vec<TokenType>) -> Result<bool, Error>{

    if let Some(token) = tokens.get(*current_index){
        let token = get_current_token(tokens, current_index)?;

        if check_tokens.iter().map(|x| token.check_against_token_type(x.clone())).any(|x| x == true){
           
            Ok(true)

        } else {
            return Ok(false)
        }

    } else {
        Err(Error::Nil)
    }

}


//operator precedence is 
// =
// ==/!=
// and/or/xor 
// >/>=/</<=
// +/-
// *// 
// !/- 
// FieldCall
// literals/functions/arrays/objects

fn expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    assign(tokens, current_index)
}

fn assign(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    let left = equality(tokens, current_index)?;


    if match_tokens(tokens, current_index, vec![
            TokenType::EQ
    ])? {
        consume_token(tokens, current_index)?;
        
        let right = assign(tokens, current_index)?;

        return Ok(Expression::Assign{
            target : Box::new(left),
            value : Box::new(right)
        })
    }
    

    Ok(left)   
}

fn equality(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    let left = logical(tokens, current_index)?;


    if match_tokens(tokens, current_index, vec![
        TokenType::EQEQ, 
        TokenType::NEQ
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;
        
        let right = equality(tokens, current_index)?;

        return Ok(Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        })
    }
    

    Ok(left)   
}

fn logical(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    let left = comp(tokens, current_index)?;


    if match_tokens(tokens, current_index, vec![
        TokenType::XOR, 
        TokenType::OR,
        TokenType::AND, 
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;
        
        let right = logical(tokens, current_index)?;

        return Ok(Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        })
    }
    

    Ok(left)   
}

fn comp(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    let left = term(tokens, current_index)?;


    if match_tokens(tokens, current_index, vec![
        TokenType::GEQ, 
        TokenType::GE,
        TokenType::LEQ, 
        TokenType::LE,
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;
        
        let right = comp(tokens, current_index)?;

        return Ok(Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        })
    }
    

    Ok(left)   
}

fn term(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    
    let left = factor(tokens, current_index)?;


    if match_tokens(tokens, current_index, vec![
        TokenType::PLUS, 
        TokenType::MINUS
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;
        
        let right = term(tokens, current_index)?;

        return Ok(Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        })
    }
    

    Ok(left)
}

fn factor(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    let left = unary(tokens, current_index)?;

    if match_tokens(tokens, current_index, vec![
        TokenType::STAR, 
        TokenType::SLASH
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;


        let right = factor(tokens, current_index)?;

        return Ok(Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        })
    }
    

    Ok(left)
}

fn unary(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    if match_tokens(tokens, current_index, vec![
            TokenType::STAR, 
            TokenType::BANG
    ])? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;

        let right = unary(tokens, current_index)?;

        return Ok(Expression::Unary{
            operator,
            right : Box::new(right)
        })
    }

    call(tokens, current_index)
}

fn call(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    let left = primary(tokens, current_index)?;


    //field call using the [] operator, this is also used for array indexing
    if match_tokens(tokens, current_index, vec![
        TokenType::LBRACK,
    ])? {
        consume_token(tokens, current_index)?;
        let right = call(tokens, current_index)?;

        //match for the closing delimiter
        match_token(tokens, current_index, TokenType::RBRACK)?;

        return Ok(Expression::FieldCall{
            target : Box::new(left),
            value : Box::new(right)
        })
    }

    //function call using the () operator
    if match_tokens(tokens, current_index, vec![
        TokenType::LPAREN,
    ])? {
        consume_token(tokens, current_index)?;
  
        let mut arguments : Vec<Expression> = Vec::new();

        while let Some(token) = tokens.get(*current_index){
           
            let argument = expr(tokens, current_index)?;
   
            arguments.push(argument);


            if let Some(next_token) = tokens.get(*current_index + 1){
                if get_current_token(tokens, current_index)?.r#type == TokenType::RPAREN { 
                    break;
                }
                if next_token.r#type == TokenType::RPAREN {
                    let current_token = get_current_token(tokens, current_index)?; 
                    return Err(Error::UnexpectedTokenOfMany{
                        unexpected : current_token.r#type,
                        expected : vec![]
                    })
                }
            }

            match_token(tokens, current_index, TokenType::COMMA)?;
        }

        //match for closing delimiter
        match_token(tokens, current_index, TokenType::RPAREN)?;



        return Ok(Expression::FunctionCall{
            function : Box::new(left),
            arguments
        })
    }

     //field call using the [] operator, this is also used for array indexing
    if match_tokens(tokens, current_index, vec![
        TokenType::DOT,
    ])? {
        consume_token(tokens, current_index)?;
        let right = call(tokens, current_index)?;

        return Ok(Expression::FieldCall{
            target : Box::new(left),
            value : Box::new(right)
        })
    }
    

    Ok(left)
}

fn primary(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
   
    let token = get_current_token(tokens, current_index)?;
    consume_token(tokens, current_index)?;
    match token.r#type{
        
        TokenType::ID(name) => {
            Expression::LiteralID(name).expr()
        },
        TokenType::NUM(number) =>{
            Expression::LiteralNum(number.parse::<f64>().unwrap()).expr()
        },
        TokenType::STR(string) => {
            Expression::LiteralStr(string).expr()
        },
        TokenType::TRUE => {
            Expression::LiteralBool(true).expr()
        },
        TokenType::FALSE => {
            Expression::LiteralBool(false).expr()
        },

        TokenType::LPAREN => {
            let expression = expr(tokens, current_index)?;
            match_token(tokens, current_index, TokenType::RPAREN)?;  

            Ok(Expression::Grp{
                inner : Box::new(expression)
            })
        },


    
        

        _ => {
            Err(Error::Nil) 
        }
    }

}

pub fn parse(tokens : Vec<Token>) -> Result<Vec<Expression>, Error> {

    let mut expressions = Vec::new();
    let index = &mut 0;

    while let Some(token) = tokens.get(*index){
        if let TokenType::EOF = token.r#type {
            return Ok(expressions) 
        }
    
        let expression = expr(&tokens, index)?;

        match_token(&tokens, index, TokenType::SEMICOLON)?;

        expressions.push(expression)
    }


    Ok(expressions)
}
