use crate::ast::Ast;


pub fn parse_code(code: &str) -> Ast {
    Ast{
        body: vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::ast::{Ast, Stmt, AstItem, FunStmt};

    use super::parse_code;

    #[test]
    fn test_simple_function_parsing() {
        let code = r#"
fun say_hello(your_name) {
    "Hello " + your_name
}"#;  
        
        let ast = parse_code(code);

        assert_eq!(ast, Ast{
            body: vec![
                AstItem::Stmt(Stmt::Fun(FunStmt{ name: "say_hello".to_string() }))
            ]
        })
    }
}