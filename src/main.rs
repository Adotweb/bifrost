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
    

            overload + (a : vec3, b : vec3) -> vec3{
                return vec3 {
                    x : a.x + b.x,
                    y : a.y + b.y,
                    z : a.z + b.z
                };
            }
            
            
        "#;
    
    let tokens = lex(text);

    //println!("{:#?}", tokens.clone());

    let ast = parse(tokens);

    

    println!("{:#?}", ast);
}
