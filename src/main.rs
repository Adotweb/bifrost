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

        type v3 = {
            x : number,
            y : number,
            z : number,
        };
    
        overload + (a : v3, b : v3) -> number {
            return a.x + b.x;
        }

            
        "#;
    
    let tokens = lex(text);

    //println!("{:#?}", tokens.clone());

    let ast = parse(tokens);

    

    println!("{:#?}", ast);
}
