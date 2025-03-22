use crate::*;

#[derive(Debug)]
pub enum Error{
    Nil,
    UnexpectedToken{
        expected : TokenType,
        unexpected : TokenType
    }
}


