use std::process::Command;

use pest_generator::derive_parser;
use quote::quote;

fn main() {
    let t = quote! {
        #[derive(Parser)]
        #[grammar = "grammar.pest"]
        pub struct FlexscriptParser;  
    };

    let t = derive_parser(t, false);

    let mut t = t.to_string();

    t = "use super::FlexscriptParser;".to_string() + &t;

    std::fs::write(
        "./packages/parser/src/parser_gen.rs", 
        t.to_string().as_bytes()).unwrap();

    // Command::new("rustfmt")
    //     .arg("./packages/parser/src/parser_gen.rs")
    //     .output().unwrap();
}
