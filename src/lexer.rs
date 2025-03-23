#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType{
    LPAREN, 
    RPAREN,
    LBRACK,
    RBRACK,
    LBRACE,
    RBRACE,


    ID(String),
    NUM(String),
    STR(String),
    FALSE,
    TRUE,
    NIL,
  

    //these are to check whether or not two tokens have the same type
    ID_,
    NUM_,
    STR_,

    COLON,
    SEMICOLON,

    DOT,
    COMMA,
    
    PLUS, 
    MINUS,
    STAR, 
    SLASH,

    BANG,
    NEQ,
    EQEQ,
    EQ,
    GEQ,
    GE,
    LEQ,
    LE,

    AND,
    OR,
    XOR,

    ARROW, // this thing: "->"
    IMPL, //this thing: "=>"
          
    FN,
    LET,
    CONST,
 
    IF,
    ELSE,

    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    RETURN,

    TO,

    OVERLOAD,

    EOF,
}

impl TokenType{
    pub fn token(&self, line : usize, column : usize) -> Token{
        return Token{
            r#type : self.clone(),
            position : (line, column)
        }
    }

    pub fn ignore_value(&self) -> Self{
        match self {
            Self::NUM(_) => Self::NUM_,
            Self::ID(_) => Self::ID_,
            Self::STR(_) => Self::STR_,
            _ => self.clone()
        }
    }

    //checks if the value is type of the value
    pub fn type_of(&self, check_type : TokenType) -> bool{
        return self.ignore_value() == check_type.ignore_value()
    }
    
    pub fn get_id_val(&self) -> Option<String>{
        if let TokenType::ID(str) = self{
            Some(str.to_string())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token{
    pub r#type : TokenType,
    pub position : (usize, usize)
}

impl Token{
    pub fn append_to(&self, tokens : &mut Vec<Token>){
        tokens.push(self.clone())
    }

    pub fn check_against_token_type(&self, check_token_type : TokenType) -> bool{ 
        return self.r#type.type_of(check_token_type)
    }
}

fn number(chars : Vec<String>, index : &mut usize, line : &mut usize, column : &mut usize, tokens : &mut Vec<Token>) -> Result<(), String>{
    
    let mut number = "".to_string();

    let start_position = (line.clone(), column.clone());

    let number_match = "_0123456789";
    
    let mut dot_used = false;

    while let Some(char) = chars.get(*index){
        if number_match.contains(char){
          
            number += char;
                
            *column += 1;
            *index += 1;

        } else if char == "."{
            if let Some(next_char) = chars.get(*index + 1){
                   

                //we have to check whether the next thing after a dot is a number, because in
                //theory a method could come there and then we dont have to throw an error
                if number_match.contains(next_char){ 
                    if dot_used {
                        return Err("Dot can only be used once in numbers!".to_string())
                    }
                    
                    number += char;

                    dot_used = true;
                } else {
                      
                    //reduce the index by one so the dot is not consumed when the next char is not
                    //a number
                    *index -= 1;
                    *column -= 1;
                    break;
                }

                    
                *column += 1;
                *index += 1;
            }
        } else {
            
            //reduce the index by one so the last character is not consumed
            *index -= 1;
            *column -= 1;
            break;
        }
    }

    //remove all "_" because they are only for readability
    let new_number = number.chars().filter(|x| *x != '_').collect::<String>();

    TokenType::NUM(new_number)
        .token(start_position.0, start_position.1)
        .append_to(tokens);
        

    Ok(())
}

fn string(chars : Vec<String>, index : &mut usize, line : &mut usize, column : &mut usize, tokens : &mut Vec<Token>) -> Result<(), String>{
    
    let mut string = "".to_string();
   
    let start_position = (line.clone(), column.clone());

    while let Some(char) = chars.get(*index){

        if char == r#"""#{ 
            break;
        }

        string += char;

        if char == "\n"{
            *line += 1;
        } 

        *column += 1;
        *index += 1;
    }

    //append the string token to the
    TokenType::STR(string)
        .token(start_position.0, start_position.1)
        .append_to(tokens);


    Ok(())
}

fn identifier(chars : Vec<String>, index : &mut usize, line : &mut usize, column : &mut usize, tokens : &mut Vec<Token>) -> Result<(), String>{

    let id_match = "abcdefghijklmnopqrstuvxyz_0123456789";

    let mut identifier = "".to_string();

    let start_position = (line.clone(), column.clone());

    while let Some(char) = chars.get(*index){

        if id_match.contains(char) || id_match.to_uppercase().contains(char){

            identifier += char;

            *index += 1;
            *column += 1;
            
        } else {

            //reduce the index by one so the last character is not consumed
            *index -= 1;
            *column -= 1;
            break;
        }


    }    

    match identifier.as_str() {
        "fn" => TokenType::FN,
        "let" => TokenType::LET,
        "const" => TokenType::CONST,

        "false" => TokenType::FALSE,
        "true" => TokenType::TRUE,
        "nil" => TokenType::NIL,

        "and" => TokenType::AND,
        "or" => TokenType::OR,
        "xor" => TokenType::XOR,
    
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,

        "while" => TokenType::WHILE,
        "for" => TokenType::FOR,
        "break" => TokenType::BREAK,
        "continue" => TokenType::CONTINUE,
        "return" => TokenType::RETURN,
        
        "overload" => TokenType::OVERLOAD,

        


         _ => TokenType::ID(identifier)
    }
        .token(start_position.0, start_position.1)
        .append_to(tokens);



    Ok(())
} 

pub fn lex(text : &'static str) -> Vec<Token>{

    let mut tokens = Vec::new();

    let characters : Vec<String> = text.chars().map(|x| x.to_string()).collect();

    let mut index = 0;
    let mut line = 0;
    let mut column = 0;

    let id_start_match = "_abcdefghijklmnopqrstuvxyz";
    let num_start_match = "0123456789"; 


    while let Some(char) = characters.get(index){
        
        let mut next_char : Option<&str> = None;

        if let Some(next) = characters.get(index + 1){
            next_char = Some(next);
        }

        match char.as_str(){
            "(" => TokenType::LPAREN.token(line, column).append_to(&mut tokens),
            ")" => TokenType::RPAREN.token(line, column).append_to(&mut tokens),
            "[" => TokenType::LBRACK.token(line, column).append_to(&mut tokens),
            "]" => TokenType::RBRACK.token(line, column).append_to(&mut tokens),
            "{" => TokenType::LBRACE.token(line, column).append_to(&mut tokens),
            "}" => TokenType::RBRACE.token(line, column).append_to(&mut tokens),
            

            "+" => TokenType::PLUS.token(line, column).append_to(&mut tokens),
            "-" => {
                match next_char{
                    Some(">") => TokenType::ARROW.token(line, column).append_to(&mut tokens),
                    _ => TokenType::MINUS.token(line, column).append_to(&mut tokens),
                }
            },
            "*" => TokenType::STAR.token(line, column).append_to(&mut tokens),
            "/" => TokenType::SLASH.token(line, column).append_to(&mut tokens),

            "," => TokenType::COMMA.token(line, column).append_to(&mut tokens),
            ":" => TokenType::COLON.token(line, column).append_to(&mut tokens),
            "." => {
                match next_char {
                    Some(char) => { 
                        if num_start_match.contains(char){
                            let _ = number(characters.clone(), &mut index, &mut line, &mut column, &mut tokens); 
                        } else {
                            TokenType::DOT.token(line, column).append_to(&mut tokens)
                        }
                    },
                    _ => TokenType::DOT.token(line, column).append_to(&mut tokens)
                }
            },
            ";" => TokenType::SEMICOLON.token(line, column).append_to(&mut tokens),

            "!" => {
                match next_char{
                    Some("=") => {
                        index += 1;
                        TokenType::NEQ.token(line, column).append_to(&mut tokens); 
                    },
                    _ => TokenType::BANG.token(line, column).append_to(&mut tokens),
                } 
            }

            "=" => {
                match next_char{
                    Some("=") => {
                        index += 1;
                        TokenType::EQEQ.token(line, column).append_to(&mut tokens)
                    },
                    Some(">") => {
                        index += 1;
                        TokenType::IMPL.token(line, column).append_to(&mut tokens)
                    },
                    _ => TokenType::EQ.token(line, column).append_to(&mut tokens),
                }
            }

            "<" => {
                match next_char{
                    Some("=") => {
                        index += 1;
                        TokenType::LEQ.token(line, column).append_to(&mut tokens)
                    },
                    _ => TokenType::LE.token(line, column).append_to(&mut tokens),
                }
            },

            ">" => {
                match next_char{
                    Some("=") => {
                        index += 1;
                        TokenType::GEQ.token(line, column).append_to(&mut tokens)
                    },
                    _ => TokenType::GE.token(line, column).append_to(&mut tokens),
                }
            }

            "\n" => { line += 1 } 


            _ => ()
        }


        if id_start_match.contains(char) || id_start_match.to_uppercase().contains(char){
            let _ = identifier(characters.clone(), &mut index, &mut line, &mut column, &mut tokens);
        }

        if num_start_match.contains(char) {
            let _ = number(characters.clone(), &mut index, &mut line, &mut column, &mut tokens);
        }

        if char == "\""{
            //we need to consume the " symbol so we dont immediately end the string
            index += 1;
            let _ = string(characters.clone(), &mut index, &mut line, &mut column, &mut tokens);
        }
        
        column += 1;
        index += 1;

    }

    TokenType::EOF.token(line, column).append_to(&mut tokens);

    tokens
}
