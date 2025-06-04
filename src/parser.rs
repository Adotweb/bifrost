use std::env::current_dir;

use crate::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type{
    NullType,
    AnyType,


    NumType,
    StrType,
    BoolType,

    CustomType(String),
   
    ArrayType(Box<Type>),
    UnionType(Vec<Type>),
    ObjectType{
        keys : Vec<String>,
        types : Vec<Type>
    },

    //the struct is somewhat special. while it has a type it is used as a normal expression rather
    //than a typed expression like the other types
    Struct{
        keys : Vec<String>,
        types : Vec<Type>
    },

    FunctionType{
        arguments : Vec<Type>,
        returns : Box<Type>
    }


}

impl Type{
    pub fn append_union_option(&self, option : Self) -> Self{
        match self{
            Type::UnionType(options) => {
                let mut new_options = options.clone();
                new_options.push(option);

                Type::UnionType(new_options)
            },
            _ => {
                let mut new_options = vec![self.clone()];

                new_options.push(option);
                Type::UnionType(new_options)
            }
        }      
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedName{
    name : Token,
    r#type : Type
}

type FallibleType = Result<Type, Error>;

//this is for typings in let and so on
fn typed_primary(tokens : &Vec<Token>, current_index : &mut usize) -> Result<TypedName, Error>{
    let name = get_current_token(tokens, current_index)?;
    
    consume_token(tokens, current_index)?;

    if match_tokens(tokens, current_index, vec![
        TokenType::COLON
    ])?{
        consume_token(tokens, current_index)?;
        let constructed = typed(tokens, current_index)?;

        Ok(TypedName{
            name,
            r#type : constructed
        })   
    } else {
        Ok(TypedName{
            name,
            r#type : Type::AnyType
        })
    }



}

fn typed(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleType{
    let mut left = Type::AnyType;
    let mut changed = false;

    while let Some(token) = tokens.get(*current_index){

         match &token.r#type {
            TokenType::BAR =>{
                consume_token(tokens, current_index)?;
                let new_left = left.append_union_option(typed(tokens, current_index)?);

                left = new_left
            } 
            TokenType::LPAREN => {
                left = grp_typed(tokens, current_index)?;
            },
            TokenType::LBRACK => {
                match_token(tokens, current_index, TokenType::LBRACK)?;
                match_token(tokens, current_index, TokenType::RBRACK)?;
                
                left = Type::ArrayType(Box::new(left))
            },
            TokenType::LBRACE => {
                left = object_typed(tokens, current_index)?;
            },
            TokenType::ID(id_type) => {
                //we cannot override a type with another type 
                //constructions like type1 type2 do not make any sense and do not work
                //so we check if the type has already been changed and if yes return
                
                if changed{
                    return Ok(left)
                } else {
                    changed = true
                }

                consume_token(tokens, current_index)?;

                match id_type.as_str() {
                    "bool" =>{
                        left = Type::BoolType;
                    },
                    "string" =>{
                        left = Type::StrType;
                    },
                    "num" =>{
                        left = Type::NumType;
                    },
                    "nil" => {
                        left = Type::NullType;
                    },
                    "any" => {
                        left = Type::AnyType;
                    }
                    _ => {
                        left = Type::CustomType(id_type.to_string())
                    }
                }

            },
            TokenType::FN => {
                left = function_typed(tokens, current_index)?;
            }
            _ => {
                return Ok(left)
            }
        }
    }

    Ok(left)
}

fn function_typed(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleType{
    match_token(tokens, current_index, TokenType::FN)?;

    match_token(tokens, current_index, TokenType::LPAREN)?;

    let mut arguments : Vec<Type> = Vec::new();
    
    let mut trailing_comma : Option<Token> = None;

    while let Some(token) = tokens.get(*current_index){

        if match_tokens(tokens, current_index, vec![
            TokenType::RPAREN
        ])? {
            consume_token(tokens, current_index)?;
            break; 
        }

        let argument_type = typed(tokens, current_index)?;
        arguments.push(argument_type);
        trailing_comma = None;

        if match_tokens(tokens, current_index, vec![
            TokenType::RPAREN
        ])? {
            consume_token(tokens, current_index)?;
            break; 
        }



        if match_tokens(tokens, current_index, vec![
            TokenType::COMMA
        ])? {
            trailing_comma = Some(get_current_token(tokens, current_index)?);
            consume_token(tokens, current_index)?;
        }

    }

    if let Some(comma_token) = trailing_comma{
        return Err(Error::UnexpectedTokenOfMany{
            expected : vec![],
            unexpected : comma_token
        })
    }

    match_token(tokens, current_index, TokenType::ARROW)?;

    let returns = typed(tokens, current_index)?;

    Ok(Type::FunctionType{
        arguments,
        returns : Box::new(returns)
    })
}

fn grp_typed(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleType{
    match_token(tokens, current_index, TokenType::LPAREN)?;
    let construct = typed(tokens, current_index)?;
    match_token(tokens, current_index, TokenType::RPAREN)?;
    return Ok(construct)
}

fn object_typed(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleType{ 
    match_token(tokens, current_index, TokenType::LBRACE)?;

    let mut keys = Vec::new();
    let mut types = Vec::new();

    while let Some(token) = tokens.get(*current_index){
        
        let name = get_current_token(tokens, current_index)?; 
        match_token(tokens, current_index, TokenType::ID_)?;

        match_token(tokens, current_index, TokenType::COLON)?;

        let construction = typed(tokens, current_index)?;

        keys.push(name.r#type.get_id_val().unwrap());
        

        println!("{:?}", construction.clone());
        types.push(construction);



        let mut comma_used = false;
        if match_tokens(tokens, current_index, vec![
            TokenType::COMMA,
        ])? {
            consume_token(tokens, current_index)?;
            comma_used = true;
        }

        if match_tokens(tokens, current_index, vec![
            TokenType::RBRACE,
        ])? {
            consume_token(tokens, current_index)?;

            return Ok(Type::ObjectType{
                keys,
                types
            })
        }

        if !comma_used {
            match_token(tokens, current_index, TokenType::COMMA)?;
        }
    }

    return Ok(Type::ObjectType{
            keys,
            types
    })

}


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

    While{
        condition : Box<Expression>,
        block : Box<Expression>
    },

    For{
        condition : Box<Expression>,
        block : Box<Expression>
    },

    Fn{
        name : Option<Token>,
        arguments : Vec<TypedName>,
        result : Option<Type>,
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
        name : TypedName,
        value : Box<Expression>,
        constant : bool
    },
    TypeDeclaration{
        name : Token,
        r#type : Type 
    },
    StructDeclaration{
        name : Token, 
        r#type : Type
    },
    StructUsage{
        struct_name : Token,
        fields : Vec<Token>,
        values : Vec<Expression>
    },

    Overload{
        operation : Token,
        arguments  : Vec<TypedName>,
        result : Type,
        body: Box<Expression>
    },

    Return(Box<Expression>),
    Break,
    Continue,
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
                unexpected : token
            })
        }

    } else {
        Err(Error::UnexpectedToken{
            expected : check_token,
            unexpected : get_current_token(tokens, current_index)?
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
        TokenType::TYPE => type_declaration(tokens, current_index),
        TokenType::STRUCT => struct_declaration(tokens, current_index),
        TokenType::FN => fn_expr(tokens, current_index),
        TokenType::OVERLOAD => overload_expr(tokens, current_index),
        TokenType::CONTINUE => Expression::Continue.expr(),
        TokenType::BREAK => Expression::Break.expr(),
        TokenType::RETURN => return_expr(tokens, current_index),
        _ => assign(tokens, current_index)
    }

}

fn return_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    let returned = expr(tokens, current_index)?;

    Expression::Return(Box::new(returned)).expr()
}

fn type_declaration(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    let name = get_current_token(tokens, current_index)?;

    match_token(tokens, current_index, TokenType::ID_)?;
    match_token(tokens, current_index, TokenType::EQ)?;

    let associated_type = typed(tokens, current_index)?;


    Expression::TypeDeclaration{
        name,
        r#type : associated_type    
    }.expr()
}

fn struct_declaration(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    let name = get_current_token(tokens, current_index)?;

    match_token(tokens, current_index, TokenType::ID_)?;
   

    let r#type = object_typed(tokens, current_index)?; 


    Expression::StructDeclaration{
        name,
        r#type : r#type,
    }.expr()
}

fn overload_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;

    let mut operator = None;

    if match_tokens(&tokens, current_index, vec![
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::BANG,
        TokenType::STAR,
        TokenType::SLASH,
        TokenType::GEQ,
        TokenType::GE,
        TokenType::LEQ,
        TokenType::LE,
        TokenType::EQEQ,
        TokenType::NEQ,
    ])?{
        operator = Some(get_current_token(tokens, current_index)?);
        consume_token(tokens, current_index)?;
    }  

    //this part is one to one correspondand with the function definitions
    
    match_token(tokens, current_index, TokenType::LPAREN)?;

    let mut arguments : Vec<TypedName> = vec![];
    

    while let Some(token) = tokens.get(*current_index){
    
        if let TokenType::RPAREN = token.r#type{
            consume_token(tokens, current_index)?;
            break; 
        }

        let argument = typed_primary(tokens, current_index)?;
        arguments.push(argument);
    
        if match_tokens(tokens, current_index, vec![
            TokenType::COMMA
        ])? {
            consume_token(tokens, current_index)?;
            let current_token = get_current_token(tokens, current_index)?;
            if current_token.r#type == TokenType::RPAREN{
                return Err(Error::UnexpectedTokenOfMany{
                    expected : vec![],
                    unexpected : current_token
                })
            }
           
            continue;
        }

    }


    let mut result_type : Option<Type> = None;
    //maybe there is a type definition
    if match_tokens(tokens, current_index, vec![
        TokenType::ARROW
    ])? {
        consume_token(tokens, current_index)?;

        result_type = Some(typed(tokens, current_index)?);

    }

    let body = expr(tokens, current_index)?;

    Ok(Expression::Overload{
        operation : operator.unwrap(),
        arguments,
        result : result_type.unwrap(),
        body : Box::new(body)
    })
    

}

fn fn_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?; 

    let mut name = None;

    if match_tokens(tokens, current_index, vec![
        TokenType::ID_
    ])?{
        name = Some(get_current_token(tokens, current_index)?); 
        consume_token(tokens, current_index)?;
    }


    match_token(tokens, current_index, TokenType::LPAREN)?;

    let mut arguments : Vec<TypedName> = vec![];
    

    while let Some(token) = tokens.get(*current_index){
    
        if let TokenType::RPAREN = token.r#type{
            consume_token(tokens, current_index)?;
            break; 
        }

        let argument = typed_primary(tokens, current_index)?;
        arguments.push(argument);
    
        if match_tokens(tokens, current_index, vec![
            TokenType::COMMA
        ])? {
            consume_token(tokens, current_index)?;
            let current_token = get_current_token(tokens, current_index)?;
            if current_token.r#type == TokenType::RPAREN{
                return Err(Error::UnexpectedTokenOfMany{
                    expected : vec![],
                    unexpected : current_token
                })
            }
           
            continue;
        }

    }


    let mut result_type : Option<Type> = None;
    //maybe there is a type definition
    if match_tokens(tokens, current_index, vec![
        TokenType::ARROW
    ])? {
        consume_token(tokens, current_index)?;

        result_type = Some(typed(tokens, current_index)?);

    }

    let body = expr(tokens, current_index)?;

    Ok(Expression::Fn{
        arguments,
        name,
        result : result_type,
        body : Box::new(body)
    })
}
    
fn let_expr(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{
    consume_token(tokens, current_index)?;
    
    let name = typed_primary(tokens, current_index)?;
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
  
    let name = typed_primary(tokens, current_index)?;
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
                    let current_token = get_current_token(tokens, current_index)?;
                    //check if we have a trailing comma
                    if current_token.r#type == TokenType::RPAREN{
                        return Err(Error::UnexpectedTokenOfMany{
                            expected : vec![], 
                            unexpected : current_token
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
    match &token.r#type{
        
        TokenType::ID(name) => {

            //clone the name for clean borrow;
            let name = name.clone();
            let struct_name = token.clone();
            if match_tokens(tokens, current_index, vec![
                TokenType::LBRACE
            ])?{
                //this means that we are using a struct
                consume_token(tokens, current_index)?; 

                let mut fields = vec![];
                let mut values = vec![];
                while let Some(token) = tokens.get(*current_index){
                   
                    let field_name = get_current_token(tokens, current_index)?;
                    match_token(tokens, current_index, TokenType::ID_)?;
                    fields.push(field_name);

                    match_token(tokens, current_index, TokenType::COLON)?; 
                    let value = expr(tokens, current_index)?;
                    values.push(value);
                    
                    println!("{:?}", get_current_token(tokens, current_index));
               
                
                    //check if we have a comma (could in theory be voluntary)
                    let mut comma_used = false;
                    if match_tokens(tokens, current_index, vec![
                        TokenType::COMMA,
                    ])? {
                        comma_used = true;
                        consume_token(tokens, current_index)?;
                    }

                    if match_tokens(tokens, current_index, vec![
                        TokenType::RBRACE,
                    ])? {
                        consume_token(tokens, current_index)?;
                        return Expression::StructUsage{ struct_name, fields, values}.expr()
                    }
                   
                    //if no comma was used so far we check again for one to make sure
                    if !comma_used{
                        match_token(tokens, current_index, TokenType::COMMA)?;
                    }

                }

            }

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
            Expression::LiteralStr(string.to_string()).expr()
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
            

            //we check for an rbrace if we find one we immediately return 
            if match_tokens(tokens, current_index, vec![TokenType::RBRACE])? {
                consume_token(tokens, current_index)?;
                return Expression::Block{
                    expressions : vec![]
                }.expr();
            };



             
                 
            block(tokens, current_index)     
        

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

fn block(tokens : &Vec<Token>, current_index : &mut usize) -> FallibleExpression{

    let mut expressions = Vec::new();

    while let Some(token) = tokens.get(*current_index){
        if let TokenType::EOF = token.r#type  {
            return Expression::Block{
                expressions
            }.expr()
        }

        if let TokenType::RBRACE = token.r#type  {
            consume_token(tokens, current_index)?;
            return Expression::Block{
                expressions
            }.expr()
        }
    
        let expression = expr(&tokens, current_index)?;


        match expression.clone() {
            Expression::StructDeclaration { name, r#type } => { match_optional_token(tokens, current_index, TokenType::SEMICOLON)? },
            Expression::Block { expressions }  => { match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?; },
            Expression::If { condition, if_block, else_if_blocks, else_block }  => { match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?; },
            Expression::While { condition, block } => { match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?; },
            Expression::For { condition, block } => { match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?; }, 
            Expression::Fn { name, arguments, body, result } => {
                if let Expression::Block { expressions } = *body {
                    match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?;
                }
            },
            Expression::Overload { operation, arguments, result, body } => {
                if let Expression::Block { expressions } = *body {
                    match_optional_token(&tokens, current_index, TokenType::SEMICOLON)?;
                }
            }
            _ => { match_token(&tokens, current_index, TokenType::SEMICOLON)?; }
        };
    

        expressions.push(expression)
    }


    Expression::Block{
        expressions
    }.expr()
 
}

fn match_optional_token(tokens : &Vec<Token>, current_index : &mut usize, optional_token : TokenType) -> Result<(), Error>{
    if match_tokens(tokens, current_index, vec![optional_token])? {
        consume_token(tokens, current_index)?;
    } 
    Ok(())
}

pub fn parse(tokens : Vec<Token>) -> Result<Vec<Expression>, Error> {

    let index = &mut 0;


    if let Expression::Block { expressions } = block(&tokens, index)?{
        return Ok(expressions)
    };

    Err(Error::Nil)
}
