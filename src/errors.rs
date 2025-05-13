use crate::*;

#[derive(Debug)]
pub enum Error{
    Nil,
    UnexpectedToken{
        expected : TokenType,
        unexpected : Token
    },
    UnexpectedTokenOfMany{
        expected : Vec<TokenType>,
        unexpected : Token
    },
    TypeNotFound
}


