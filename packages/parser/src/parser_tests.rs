use crate::parse_text;

#[cfg(test)]
mod assingment_tests {
    use super::*;

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
}

#[cfg(test)]
mod identifier_tests {
    use super::*;
    
    #[test]
    fn test_identifier() {
        let code = "const makkara_perunat = 5";
    
        let ast = parse_text(code).unwrap();
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn test_int_literal() {
        let code = "5";

        let ast = super::parse_text(code).unwrap();
    }

    #[test]
    fn test_float_literal() {
        let code = "5.5";

        let ast = super::parse_text(code).unwrap();
    }

    #[test]
    fn test_string_literal() {
        let code = r#""hello""#;

        let ast = super::parse_text(code).unwrap();
    }

    #[test]
    fn test_complicated_string_literal() {
        let code = r#"const a = "foo()""#;

        let ast = super::parse_text(code).unwrap();
    }
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

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn test_define_function() {
        let code = r#"const foo = () => { }"#;
    
        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_async_function() {
        let code = r#"const foo = async () => { }"#;
    
        let ast = parse_text(code).unwrap();
    }
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

#[cfg(test)]
mod match_tests {
    use super::*;

    #[test]
    fn test_match_unknown() {
        let code = r#"match a { _ => { } }"#;
    
        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_match_function_unknown() {
        let code = r#"match foo() { _ => { } }"#;
    
        let ast = parse_text(code).unwrap();
    }
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

#[cfg(test)]
mod xml_tests {
    use super::*;

    #[test]
    fn test_xml() {
        let code = r#"<div></div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_item() {
        let code = r#"<div>hello</div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_attribute() {
        let code = r#"<div class="hello"></div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_variable_child() {
        let code = r#"<div>{a}</div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_variable_attribute() {
        let code = r#"<div class={a}></div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_number_attribute() {
        let code = r#"<div class={5}></div>"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_xml_with_number_item() {
        let code = r#"<div>{5}</div>"#;

        let ast = parse_text(code).unwrap();
    }
}

#[cfg(test)]
mod call_tests {
    use super::*;

    #[test]
    fn test_call() {
        let code = r#"foo(5)"#;

        let ast = parse_text(code).unwrap();
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn test_array() {
        let code = r#"[1, 2, 3]"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_array_with_variable() {
        let code = r#"[1, 2, a]"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_empty_array() {
        let code = r#"[]"#;

        let ast = parse_text(code).unwrap();
    }
}

#[cfg(test)]
mod sql_tests {
    use super::*;

    #[test]
    fn test_sql() {
        let code = r#"const sql = select id, name from people where id == 1"#;

        let ast = parse_text(code).unwrap();
    }
}

#[cfg(test)]
mod property_access_tests {
    use super::*;

    #[test]
    fn test_property_access() {
        let code = r#"a.b"#;

        let ast = parse_text(code).unwrap();
    }

    #[test]
    fn test_property_access_with_function_call() {
        let code = r#"a.b(5)"#;

        let ast = parse_text(code).unwrap();
    }
}