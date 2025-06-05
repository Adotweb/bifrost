mod lexer;
mod parser;
mod errors;
mod types;

pub use lexer::*;
pub use parser::*;
pub use errors::*;
pub use types::*;

fn main() {


    let text = r#"
   

            struct vec3 {
                x : num,
                y : num,
                z : num
            }            
    

            
        "#;
    
    let tokens = lex(text);

    //println!("{:#?}", tokens.clone());

    let ast = parse(tokens);

    

    println!("{:#?}", ast);
}
