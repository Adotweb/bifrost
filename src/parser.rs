use crate::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {

    LiteralStr(String),
    LiteralNum(u64, u64),
    LiteralBool(bool),
    LiteralID(String),
    LiteralNil,

    LiteralArray(Vec<Expression>),
    LiteralObject(Vec<Expression>, Vec<Expression>),

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
        //have to hold the condition and the blocks 
        else_if_blocks : Vec<(Expression, Expression)>,
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
//expression functions (things like if, else, while statements etc.)
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
    
    match get_current_token(tokens, current_index)?.r#type {
        TokenType::LET => let_expr(tokens, current_index),
        TokenType::CONST => const_expr(tokens, current_index),
        TokenType::IF => if_expr(tokens, current_index),
        TokenType::WHILE => while_expr(tokens, current_index),
        TokenType::FOR => for_expr(tokens, current_index),
        _ => assign(tokens, current_index)
    }

}

fn let_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;
  
    let name = get_current_token(tokens, current_index)?;
    match_token(tokens, current_index, TokenType::ID_)?;
    match_token(tokens, current_index, TokenType::EQ)?;

    let value = expr(tokens, current_index)?;

    Expression::Declaration{
        name,
        value : Box::new(value),
        constant : false
    }.expr()
}

fn const_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;
  
    let name = get_current_token(tokens, current_index)?;
    match_token(tokens, current_index, TokenType::ID_)?;
    match_token(tokens, current_index, TokenType::EQ)?;

    let value = expr(tokens, current_index)?;

    Expression::Declaration{
        name,
        value : Box::new(value),
        constant : true
    }.expr()
}


fn if_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    //match_token(tokens, current_index, TokenType::LPAREN)?;
    let condition = expr(tokens, current_index)?;
    //match_token(tokens, current_index, TokenType::RPAREN)?;

    let if_block = expr(tokens, current_index)?;
    let mut else_if_blocks : Vec<(Expression, Expression)> = Vec::new();
    let mut else_block : Option<Box<Expression>> = None;

    match_optional_token(tokens, current_index, TokenType::SEMICOLON)?;

    while match_tokens(tokens, current_index, vec![
        TokenType::ELSE
    ])? { 
        consume_token(tokens, current_index)?;
  

        //check if we have some else ifs
        if match_tokens(tokens, current_index, vec![
            TokenType::IF
        ])? {
            //consume_token(tokens, current_index)?;
 
            
            let condition = expr(tokens, current_index)?;


            //match_token(tokens, current_index, TokenType::RPAREN)?;          
            let block = expr(tokens, current_index)?;

            match_optional_token(tokens, current_index, TokenType::SEMICOLON)?;
            else_if_blocks.push((condition, block));
        } else {
            else_block = Some(Box::new(expr(tokens, current_index)?));
        }
    }

    Expression::If{
        condition : Box::new(condition),
        if_block : Box::new(if_block),
        else_if_blocks,
        else_block
    }.expr()
}

fn while_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    let condition = expr(tokens, current_index)?;

    let block = expr(tokens, current_index)?;

    Expression::While{
        condition : Box::new(condition),
        block : Box::new(block)
    }.expr()
}

fn for_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{ 
    todo!()
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
                            unexpected : TokenType::COMMA
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

            //we break into two so we can do comparisons with hashmaps
            let integral = number.split(".").nth(0).unwrap().parse::<u64>().unwrap();
            let fractional = match number.split(".").nth(1) {
                Some(part) => part.parse::<u64>().unwrap(),
                None => 0
            };
                

            Expression::LiteralNum(integral, fractional).expr()
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


        //arrays
        TokenType::LBRACK => {
            let mut literals : Vec<Expression> = Vec::new();
            while let Some(token) = tokens.get(*current_index){
                let literal = expr(tokens, current_index)?;
                
                literals.push(literal);

                if get_current_token(tokens, current_index)?.r#type == TokenType::RBRACK{
                    consume_token(tokens, current_index)?;
                    break;
                }

                match_token(tokens, current_index, TokenType::COMMA)?;


                //we check twice to allow for trailing commas
                if get_current_token(tokens, current_index)?.r#type == TokenType::RBRACK{
                    consume_token(tokens, current_index)?;
                    break;
                }
                
            } 

            Ok(Expression::LiteralArray(literals))
        }, 

        //objects

        TokenType::LBRACE => {
            
            //first we check if we have an identifier followed by a colon 
            //if yes we toggle the object mode on
            let mut object_mode = false;

            //we check if were in object mode by searching for a colon
            //if we see it we consume it
            if match_tokens(tokens, current_index, vec![TokenType::RBRACE])? {
                consume_token(tokens, current_index)?;
                return Expression::Block{
                    expressions : vec![]
                }.expr();
            };


            let first_expr = expr(tokens, current_index)?;


            //we check if were in object mode by searching for a colon
            //if we see it we consume it
            if match_tokens(tokens, current_index, vec![TokenType::COLON])? {
                object_mode = true;
                consume_token(tokens, current_index)?;
            }
             
            if object_mode {
  
                //when were in object mode, the first wave of keys is already there and we consume
                //it
                let first_key = first_expr.clone();
                let first_expr = expr(tokens, current_index)?;
               
                

                let mut keys = Vec::new();
                let mut values = Vec::new();

                keys.push(first_key);
                values.push(first_expr);


                //then we check if we are done, and if not we check for a comma
                if match_tokens(tokens, current_index, vec![
                    TokenType::RBRACE
                ])? {
                    consume_token(tokens, current_index)?;
                    return Ok(Expression::LiteralObject(keys, values)) 
                } else {
                    match_token(tokens, current_index, TokenType::COMMA)?;
                }

                //again we match twice to lookout for trailing commas
                if match_tokens(tokens, current_index, vec![
                    TokenType::RBRACE
                ])? {
                    consume_token(tokens, current_index)?;
                    return Ok(Expression::LiteralObject(keys, values)) 
                } 


                //after that we do that again but in a loop this time
                while let Some(token) = tokens.get(*current_index){

                    let key = expr(tokens, current_index)?; 

                    match_token(tokens, current_index, TokenType::COLON)?;

                    let literal = expr(tokens, current_index)?;

                    keys.push(key);
                    values.push(literal);

                    if match_tokens(tokens, current_index, vec![
                        TokenType::RBRACE
                    ])? {
                        consume_token(tokens, current_index)?;
                        return Ok(Expression::LiteralObject(keys, values)) 
                    }

                    match_token(tokens, current_index, TokenType::COMMA)?;
                    
                    if match_tokens(tokens, current_index, vec![
                        TokenType::RBRACE
                    ])? {
                        consume_token(tokens, current_index)?;
                        return Ok(Expression::LiteralObject(keys, values)) 
                    }
                
                }

            }


            //when were not in object mode we are in block mode
            if !object_mode {
                

                let mut block : Vec<Expression> = Vec::new();
                block.push(first_expr);
                
                match_token(tokens, current_index, TokenType::SEMICOLON)?;
                if match_tokens(tokens, current_index, vec![
                        TokenType::RBRACE
                ])? {
                    consume_token(tokens, current_index)?;
                    return Expression::Block{
                        expressions : block
                    }.expr()
                }


                while let Some(token) = tokens.get(*current_index){

                    let expression = expr(tokens, current_index)?;


                    block.push(expression);

                    match_token(tokens, current_index, TokenType::SEMICOLON)?;

                    if match_tokens(tokens, current_index, vec![
                        TokenType::RBRACE
                    ])? {
                        consume_token(tokens, current_index)?;
                        break;
                    }
                }

                return Expression::Block{
                    expressions : block
                }.expr()
            }

            Ok(Expression::Block{
                expressions : vec![]
            })
        }

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

fn match_optional_token(tokens : &Vec<Token>, current_index : &mut usize, optional_token : TokenType) -> Result<(), Error>{
    if match_tokens(tokens, current_index, vec![optional_token])? {
        consume_token(tokens, current_index)?;
    } 
    Ok(())
}

pub fn parse(tokens : Vec<Token>) -> Result<Vec<Expression>, Error> {

    let mut expressions = Vec::new();
    let index = &mut 0;

    while let Some(token) = tokens.get(*index){
        if let TokenType::EOF = token.r#type {
            return Ok(expressions) 
        }
    
        let expression = expr(&tokens, index)?;


        match expression.clone() {
            Expression::Block { expressions }  => { match_optional_token(&tokens, index, TokenType::SEMICOLON)?; },
            Expression::If { condition, if_block, else_if_blocks, else_block }  => { match_optional_token(&tokens, index, TokenType::SEMICOLON)?; },
            Expression::While { condition, block } => { match_optional_token(&tokens, index, TokenType::SEMICOLON)?; },
            Expression::For { condition, block } => { match_optional_token(&tokens, index, TokenType::SEMICOLON)?; },
            _ => { match_token(&tokens, index, TokenType::SEMICOLON)?; }
        };
    

        expressions.push(expression)
    }


    Ok(expressions)
}
