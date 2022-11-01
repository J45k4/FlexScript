use crate::parser::parse_raw_ast;

#[cfg(test)]
mod assingment_tests {
    use super::*;

    #[test]
    fn test_const_assingment() {
        let code = "const a int = 5";
    
        parse_raw_ast(code).unwrap();
    }
    
    #[test]
    fn test_let_assigment() {
        let code = "let a int = 5";
    
        parse_raw_ast(code).unwrap();
    }
    
    #[test]
    fn test_assigment() {
        let code = "a = 5";
    
        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod identifier_tests {
    use super::*;
    
    #[test]
    fn test_identifier() {
        let code = "const makkara_perunat = 5";
    
        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn test_int_literal() {
        let code = "5";

        super::parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_float_literal() {
        let code = "5.5";

        super::parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_string_literal() {
        let code = r#""hello""#;

        super::parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_complicated_string_literal() {
        let code = r#"const a = "foo()""#;

        super::parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod if_tests {
    use super::*;

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
    
        parse_raw_ast(code).unwrap();
    }
    
    #[test]
    fn test_if_else_if_else() {
        let code = r#"
    if a == 5 { 
    
    } else if a == 6 {
    
    } else {
    
    }"#;
    
        parse_raw_ast(code).unwrap();
    }    
}

#[cfg(test)]
mod for_tests {
    use super::*;

    #[test]
    fn test_for() {
        let code = r#"for { }"#;
    
        parse_raw_ast(code).unwrap();
    }
    
    #[test]
    fn test_range_for() {
        let code = r#"for i in 0..10 { }"#;
    
        parse_raw_ast(code).unwrap();
    }
    
    #[test]
    fn test_for_identifier() {
        let code = r#"for i in integers { }"#;
    
        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn test_define_function() {
        let code = r#"const foo = () => { }"#;
    
        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_async_function() {
        let code = r#"const foo = async () => { }"#;
    
        parse_raw_ast(code).unwrap();
    }
}


#[test]
fn test_object_literal() {
    let code = r#"const foo = { }"#;

    parse_raw_ast(code).unwrap();
}

#[test]
fn test_empty_type_stmt() {
    let code = r#"type Person { }"#;

    parse_raw_ast(code).unwrap();
}

#[test]
fn test_scalar_type_stmt() {
    let code = r#"type BigBob int"#;

    parse_raw_ast(code).unwrap();
}

#[cfg(test)]
mod match_tests {
    use super::*;

    #[test]
    fn test_match_unknown() {
        let code = r#"match a { _ => { } }"#;
    
        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_match_function_unknown() {
        let code = r#"match foo() { _ => { } }"#;
    
        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod enum_tests {
    use super::*;

    #[test]
    fn test_empty_enum() {
        let code = r#"enum Person { }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_enum_with_single_variant() {
        let code = r#"enum Person { Bob }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_enum_with_multiple_variants() {
        let code = r#"enum Person { Bob Alice Eve }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_enum_with_single_variant_with_type() {
        let code = r#"enum Person { Bob(int) }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_enum_with_object_variant() {
        let code = r#"enum Person { Bob { name string } }"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod xml_tests {
    use super::*;

    #[test]
    fn test_xml() {
        let code = r#"<div></div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_item() {
        let code = r#"<div>hello</div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_attribute() {
        let code = r#"<div class="hello"></div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_variable_child() {
        let code = r#"<div>{a}</div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_variable_attribute() {
        let code = r#"<div class={a}></div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_number_attribute() {
        let code = r#"<div class={5}></div>"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_xml_with_number_item() {
        let code = r#"<div>{5}</div>"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod call_tests {
    use super::*;

    #[test]
    fn test_call() {
        let code = r#"foo(5)"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_function_call() {
        let code = r#"foo(5)"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn test_array() {
        let code = r#"[1, 2, 3]"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_array_with_variable() {
        let code = r#"[1, 2, a]"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_empty_array() {
        let code = r#"[]"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod sql_tests {
    use crate::parse_raw_ast;

    use super::*;

    #[test]
    fn test_sql() {
        let code = r#"const sql = select id, name from people where id == 1"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod property_access_tests {
    use super::*;

    #[test]
    fn test_property_access() {
        let code = r#"a.b"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_property_access_with_function_call() {
        let code = r#"a.b(5)"#;

        parse_raw_ast(code).unwrap();
    }
}

#[cfg(test)]
mod struct_test {
    use crate::parse_raw_ast;

    use super::*;

    #[test]
    fn test_struct() {
        let code = r#"struct Person { name string }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_struct_with_multiple_fields() {
        let code = r#"struct Human {
            name string = "qwer"
            age int = 10
            favorite_color string?
        }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_struct_with_default() {
        let code = r#"struct Person { 
            name string = "qwerty" age int 
        }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_struct_initialization() {
        let code = r#"const new_human = Human { }"#;

        parse_raw_ast(code).unwrap();
    }

    #[test]
    fn test_struct_with_fields_initialization() {
        let code = r#"const new_human = Human { name: "makkara" }"#;

        parse_raw_ast(code).unwrap();
    }
}