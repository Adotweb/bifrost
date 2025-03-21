mod lexer;
mod parser;
mod errors;


pub use lexer::*;
pub use parser::*;
pub use errors::*;

fn main() {


    let text = r#"
    
            let p = 5;

            let s = "hello there";


        "#;
    
    let tokens = lex(text);


    println!("{:#?}", tokens);
}
