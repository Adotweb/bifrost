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


//helper function to construct left associative binary expressions
fn binary(tokens : &Vec<Token>, current_index : &mut usize, matches : Vec<TokenType>, below : fn(&Vec<Token>, &mut usize) -> FallibleExpression) -> FallibleExpression {
    let mut left = below(tokens, current_index)?;

    while match_tokens(tokens, current_index, matches.clone())? {
        let operator = get_current_token(tokens, current_index)?;
        consume_token(tokens, current_index)?;


        let right = below(tokens, current_index)?;

        left = Expression::Binary{
            left : Box::new(left),
            operator,
            right : Box::new(right)
        }
    }
    

    Ok(left)
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
    binary(tokens, current_index, vec![
        TokenType::EQ,
    ], equality)   
}

fn equality(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    binary(tokens, current_index, vec![
        TokenType::EQEQ,
        TokenType::NEQ,
    ], logical)   
}

fn logical(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    binary(tokens, current_index, vec![
        TokenType::XOR,
        TokenType::OR,
        TokenType::AND,
    ], comp) 
}

fn comp(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{    
    binary(tokens, current_index, vec![
        TokenType::GEQ,
        TokenType::GE,
        TokenType::LEQ,
        TokenType::LE,
    ], term)
}

fn term(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    binary(tokens, current_index, vec![
        TokenType::PLUS,
        TokenType::MINUS,
    ], factor)
}




fn factor(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    binary(tokens, current_index, vec![
        TokenType::STAR,
        TokenType::SLASH,
    ],unary)
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
    let mut left = primary(tokens, current_index)?;

   
    while match_tokens(tokens, current_index, vec![
        TokenType::LBRACK,
        TokenType::LPAREN,
        TokenType::DOT
    ])? {

        match get_current_token(tokens, current_index)?.r#type {
            TokenType::DOT => {
                let operator = get_current_token(tokens, current_index)?;
                consume_token(tokens, current_index)?;
    

                //we use primary instead of expr because DOT cannot be used for something like: 
                //4.(3 + 4)
                //thats what the [] operator is for
                let right = primary(tokens, current_index)?;
                left = Expression::Binary{
                    left : Box::new(left), 
                    operator,
                    right :  Box::new(right)
                }
            },
            TokenType::LBRACK => {
               
                let operator = get_current_token(tokens, current_index)?;
                consume_token(tokens, current_index)?;

                let right = expr(tokens, current_index)?;

                match_token(tokens, current_index, TokenType::RBRACK)?;

                left = Expression::Binary{
                    left : Box::new(left),
                    operator,
                    right : Box::new(right)
                }
            },
            TokenType::LPAREN => {
                consume_token(tokens, current_index)?;
                
                let mut arguments : Vec<Expression> = Vec::new();
                while let Some(token) = tokens.get(*current_index){
                    let argument = expr(tokens, current_index)?; 
                    arguments.push(argument);
 
                    //check if we encountered the closing brackets
                    if get_current_token(tokens, current_index)?.r#type == TokenType::RPAREN{
                        consume_token(tokens, current_index)?;
                        break;
                    }                   

                    match_token(tokens, current_index, TokenType::COMMA)?;

                    //check if we have a trailing comma
                    if get_current_token(tokens, current_index)?.r#type == TokenType::RPAREN{
                        return Err(Error::UnexpectedTokenOfMany{
                            expected : vec![], 
                            unexpected : TokenType::RPAREN
                        })
                    }
                }

                left = Expression::FunctionCall{
                    function : Box::new(left),
                    arguments 
                }
            }
            _ => ()
        }

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
