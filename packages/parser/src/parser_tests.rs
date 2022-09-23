use crate::parse_text;


#[test]
fn test_const_assingment() {
    let code = "const a int = 5";

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_let_assigment() {
    let code = "let a int = 5";

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_assigment() {
    let code = "a = 5";

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_if() {
    let code = r#"
if a == 5 { 

}"#;
}

#[test]
fn test_if_else() {
    let code = r#"
if a == 5 { } else { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_if_else_if_else() {
    let code = r#"
if a == 5 { 

} else if a == 6 {

} else {

}"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_for() {
    let code = r#"for { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_range_for() {
    let code = r#"for i in 0..10 { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_for_identifier() {
    let code = r#"for i in integers { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_function_call() {
    let code = r#"foo(5)"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_define_function() {
    let code = r#"const foo = () => { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_object_literal() {
    let code = r#"const foo = { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_empty_type_stmt() {
    let code = r#"type Person { }"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_scalar_type_stmt() {
    let code = r#"type BigBob int"#;

    let ast = parse_text(code).unwrap();
}

#[test]
fn test_match_unknown() {
    let code = r#"match a { _ => { } }"#;

    let ast = parse_text(code).unwrap();
}

#[cfg(test)]
mod enum_tests {
    use super::*;

    #[test]
    fn test_empty_enum() {
        let code = r#"enum Person { }"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_enum_with_single_variant() {
        let code = r#"enum Person { Bob }"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_enum_with_multiple_variants() {
        let code = r#"enum Person { Bob Alice Eve }"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_enum_with_single_variant_with_type() {
        let code = r#"enum Person { Bob(int) }"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_enum_with_object_variant() {
        let code = r#"enum Person { Bob { name string } }"#;

        let ast = parse_text(code).unwrap();
    }
}