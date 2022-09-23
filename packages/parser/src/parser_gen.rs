use super::FlexscriptParser;
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    number,
    string,
    string_literal,
    identifier,
    WHITESPACE,
    COMMENT,
    plus,
    minus,
    multi,
    divide,
    logical_and,
    logical_or,
    logical_eq,
    logical_smaller,
    logical_bigger,
    logical_smaller_eq,
    logical_bigger_eq,
    logical_not_eq,
    true_bool,
    false_bool,
    bool_lit,
    factor,
    while_expr,
    for_expr,
    call_expr,
    range_expr,
    term,
    expr,
    int_type,
    bool_type,
    string_type,
    object_type,
    type_def,
    field_decorator,
    const_stmt,
    let_stmt,
    assignment_stmt,
    use_stmt,
    struct_stmt,
    enum_field_object_field,
    enum_field_object,
    enum_field_tuple,
    enum_field,
    enum_stmt,
    type_field,
    type_object,
    type_simple,
    type_stmt,
    return_stmt,
    break_stmt,
    continue_stmt,
    function_arg,
    function_args,
    function_stmt,
    object_stmt_field,
    object_stmt,
    if_branch,
    else_if_branch,
    else_branch,
    if_stmt,
    match_condition,
    match_case,
    match_stmt,
    block_stmt,
    stmt,
    stmts,
    file,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for FlexscriptParser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            #![allow(clippy::upper_case_acronyms)]
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state| {
                            state
                                .repeat(|state| super::visible::WHITESPACE(state))
                                .and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::visible::COMMENT(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    super::visible::WHITESPACE(state)
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    } else {
                        Ok(state)
                    }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn number(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::number, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::ASCII_DIGIT(state).and_then(|state| {
                                    state.repeat(|state| self::ASCII_DIGIT(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::string, |state| {
                        state.sequence(|state| {
                            state.optional(|state| {
                                self::ASCII_ALPHANUMERIC(state).and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::hidden::skip(state)
                                                .and_then(|state| self::ASCII_ALPHANUMERIC(state))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_literal(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::string_literal, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("\"")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::string(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("\""))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn identifier(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::identifier, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::ASCII_ALPHANUMERIC(state).and_then(|state| {
                                    state.repeat(|state| self::ASCII_ALPHANUMERIC(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        state
                            .match_string(" ")
                            .or_else(|state| self::NEWLINE(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        state
                            .sequence(|state| {
                                state
                                    .match_string("/*")
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| {
                                                        state.match_string("*/")
                                                    })
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| state.match_string("*/"))
                            })
                            .or_else(|state| {
                                state.sequence(|state| {
                                    state
                                        .match_string("//")
                                        .and_then(|state| state.repeat(|state| self::ANY(state)))
                                })
                            })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn plus(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::plus, |state| state.match_string("+"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn minus(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::minus, |state| state.match_string("-"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn multi(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::multi, |state| state.match_string("*"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn divide(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::divide, |state| state.match_string("/"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_and(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_and, |state| state.match_string("&&"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_or(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_or, |state| state.match_string("||"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_eq(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_eq, |state| state.match_string("=="))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_smaller(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_smaller, |state| state.match_string("<"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_bigger(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_bigger, |state| state.match_string(">"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_smaller_eq(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_smaller_eq, |state| state.match_string("<="))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_bigger_eq(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_bigger_eq, |state| state.match_string(">="))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn logical_not_eq(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::logical_not_eq, |state| state.match_string("!="))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn true_bool(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::true_bool, |state| state.match_string("true"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn false_bool(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::false_bool, |state| state.match_string("false"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bool_lit(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::bool_lit, |state| {
                        self::true_bool(state).or_else(|state| self::false_bool(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn factor(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::factor, |state| {
                        state
                            .sequence(|state| {
                                state
                                    .match_string("(")
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| self::expr(state))
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| state.match_string(")"))
                            })
                            .or_else(|state| self::number(state))
                            .or_else(|state| self::bool_lit(state))
                            .or_else(|state| self::identifier(state))
                            .or_else(|state| self::string_literal(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn while_expr(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::while_expr, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("while")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::stmt(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_expr(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::for_expr, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("for")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.optional(|state| {
                                        state.sequence(|state| {
                                            self::identifier(state)
                                                .and_then(|state| super::hidden::skip(state))
                                                .and_then(|state| state.match_string("in"))
                                                .and_then(|state| super::hidden::skip(state))
                                                .and_then(|state| {
                                                    self::range_expr(state)
                                                        .or_else(|state| self::expr(state))
                                                })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn call_expr(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::call_expr, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("("))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(")"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn range_expr(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::range_expr, |state| {
                        state.sequence(|state| {
                            state
                                .optional(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(".."))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::expr(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::term, |state| {
                        state.sequence(|state| {
                            self::factor(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::multi(state)
                                                        .or_else(|state| self::divide(state))
                                                        .and_then(|state| {
                                                            super::hidden::skip(state)
                                                        })
                                                        .and_then(|state| self::factor(state))
                                                })
                                                .and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| {
                                                                    state.sequence(|state| {
                                                                        self::multi(state)
                                                                            .or_else(|state| {
                                                                                self::divide(state)
                                                                            })
                                                                            .and_then(|state| {
                                                                                super::hidden::skip(
                                                                                    state,
                                                                                )
                                                                            })
                                                                            .and_then(|state| {
                                                                                self::factor(state)
                                                                            })
                                                                    })
                                                                },
                                                            )
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state . rule (Rule :: expr , | state | { state . sequence (| state | { self :: term (state) . and_then (| state | { super :: hidden :: skip (state) }) . and_then (| state | { state . sequence (| state | { state . optional (| state | { state . sequence (| state | { self :: plus (state) . or_else (| state | { self :: minus (state) }) . and_then (| state | { super :: hidden :: skip (state) }) . and_then (| state | { self :: term (state) }) }) . and_then (| state | { state . repeat (| state | { state . sequence (| state | { super :: hidden :: skip (state) . and_then (| state | { state . sequence (| state | { self :: plus (state) . or_else (| state | { self :: minus (state) }) . and_then (| state | { super :: hidden :: skip (state) }) . and_then (| state | { self :: term (state) }) }) }) }) }) }) }) }) }) . and_then (| state | { super :: hidden :: skip (state) }) . and_then (| state | { state . optional (| state | { state . sequence (| state | { self :: logical_and (state) . or_else (| state | { self :: logical_or (state) }) . or_else (| state | { self :: logical_eq (state) }) . or_else (| state | { self :: logical_smaller_eq (state) }) . or_else (| state | { self :: logical_bigger_eq (state) }) . or_else (| state | { self :: logical_smaller (state) }) . or_else (| state | { self :: logical_bigger (state) }) . or_else (| state | { self :: logical_not_eq (state) }) . and_then (| state | { super :: hidden :: skip (state) }) . and_then (| state | { self :: expr (state) }) }) }) }) }) . or_else (| state | { self :: while_expr (state) }) . or_else (| state | { self :: call_expr (state) }) })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn int_type(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::int_type, |state| state.match_string("int"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bool_type(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::bool_type, |state| state.match_string("bool"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn string_type(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::string_type, |state| state.match_string("string"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object_type(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::object_type, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("{")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::type_field(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_def(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::type_def, |state| {
                        self::int_type(state)
                            .or_else(|state| self::bool_type(state))
                            .or_else(|state| self::string_type(state))
                            .or_else(|state| self::object_type(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn field_decorator(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::field_decorator, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("@")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("("))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(")"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn const_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::const_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("const")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::type_def(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("="))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::stmt(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn let_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::let_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("let")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::type_def(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("="))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assignment_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::assignment_stmt, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("="))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::use_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("use")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn struct_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::struct_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("struct")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn enum_field_object_field(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::enum_field_object_field, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::type_def(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn enum_field_object(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::enum_field_object, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("{")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::enum_field_object_field(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| {
                                                                self::enum_field_object_field(state)
                                                            },
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn enum_field_tuple(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::enum_field_tuple, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("(")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::identifier(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::identifier(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(")"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn enum_field(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::enum_field, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.optional(|state| {
                                        self::enum_field_object(state)
                                            .or_else(|state| self::enum_field_tuple(state))
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn enum_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::enum_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("enum")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::enum_field(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::enum_field(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_field(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::type_field, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::type_def(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::field_decorator(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::field_decorator(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_object(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::type_object, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("{")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::type_field(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::type_field(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_simple(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::type_simple, |state| self::type_def(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::type_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("type")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::identifier(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    self::type_object(state)
                                        .or_else(|state| self::type_simple(state))
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn return_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::return_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("return")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn break_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::break_stmt, |state| state.match_string("break"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn continue_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::continue_stmt, |state| state.match_string("continue"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_arg(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::function_arg, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::type_def(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_args(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::function_args, |state| {
                        state.sequence(|state| {
                            self::function_arg(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string(",")
                                                        .and_then(|state| {
                                                            super::hidden::skip(state)
                                                        })
                                                        .and_then(|state| self::function_arg(state))
                                                })
                                                .and_then(|state| {
                                                    state.repeat(|state| {
                                                        state.sequence(|state| {
                                                            super::hidden::skip(state).and_then(
                                                                |state| {
                                                                    state.sequence(|state| {
                                                                        state
                                                                            .match_string(",")
                                                                            .and_then(|state| {
                                                                                super::hidden::skip(
                                                                                    state,
                                                                                )
                                                                            })
                                                                            .and_then(|state| {
                                                                                self::function_arg(
                                                                                    state,
                                                                                )
                                                                            })
                                                                    })
                                                                },
                                                            )
                                                        })
                                                    })
                                                })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::function_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("(")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.optional(|state| self::function_args(state))
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(")"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("=>"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object_stmt_field(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::object_stmt_field, |state| {
                        state.sequence(|state| {
                            self::identifier(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string(":"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::object_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("{")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::object_stmt_field(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::object_stmt_field(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_branch(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::if_branch, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("if")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_if_branch(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::else_if_branch, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("else if")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_branch(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::else_branch, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("else")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::if_stmt, |state| {
                        state.sequence(|state| {
                            self::if_branch(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::else_if_branch(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::else_if_branch(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::else_branch(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn match_condition(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::match_condition, |state| {
                        state.sequence(|state| {
                            self::expr(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("|"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| {
                                    state.sequence(|state| {
                                        state.optional(|state| {
                                            self::match_condition(state).and_then(|state| {
                                                state.repeat(|state| {
                                                    state.sequence(|state| {
                                                        super::hidden::skip(state).and_then(
                                                            |state| self::match_condition(state),
                                                        )
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn match_case(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::match_case, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("_")
                                .or_else(|state| self::match_condition(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("=>"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::stmt(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn match_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::match_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("match")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::expr(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("{"))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::match_case(state))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block_stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::block_stmt, |state| {
                        state.sequence(|state| {
                            state
                                .match_string("{")
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.match_string("}"))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmt(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::stmt, |state| {
                        self::for_expr(state)
                            .or_else(|state| self::const_stmt(state))
                            .or_else(|state| self::let_stmt(state))
                            .or_else(|state| self::assignment_stmt(state))
                            .or_else(|state| self::if_stmt(state))
                            .or_else(|state| self::use_stmt(state))
                            .or_else(|state| self::enum_stmt(state))
                            .or_else(|state| self::struct_stmt(state))
                            .or_else(|state| self::type_stmt(state))
                            .or_else(|state| self::return_stmt(state))
                            .or_else(|state| self::break_stmt(state))
                            .or_else(|state| self::continue_stmt(state))
                            .or_else(|state| self::function_stmt(state))
                            .or_else(|state| self::range_expr(state))
                            .or_else(|state| self::object_stmt(state))
                            .or_else(|state| self::match_stmt(state))
                            .or_else(|state| self::block_stmt(state))
                            .or_else(|state| self::expr(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn stmts(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::stmts, |state| {
                        state.sequence(|state| {
                            state.optional(|state| {
                                self::stmt(state).and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            super::hidden::skip(state)
                                                .and_then(|state| self::stmt(state))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn file(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::file, |state| {
                        state.sequence(|state| {
                            self::SOI(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::stmts(state)))
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| self::EOI(state))
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHANUMERIC(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state
                        .match_range('a'..'z')
                        .or_else(|state| state.match_range('A'..'Z'))
                        .or_else(|state| state.match_range('0'..'9'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(
                    state: ::std::boxed::Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<Rule>>>
                {
                    state
                        .match_string("\n")
                        .or_else(|state| state.match_string("\r\n"))
                        .or_else(|state| state.match_string("\r"))
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::number => rules::number(state),
            Rule::string => rules::string(state),
            Rule::string_literal => rules::string_literal(state),
            Rule::identifier => rules::identifier(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::plus => rules::plus(state),
            Rule::minus => rules::minus(state),
            Rule::multi => rules::multi(state),
            Rule::divide => rules::divide(state),
            Rule::logical_and => rules::logical_and(state),
            Rule::logical_or => rules::logical_or(state),
            Rule::logical_eq => rules::logical_eq(state),
            Rule::logical_smaller => rules::logical_smaller(state),
            Rule::logical_bigger => rules::logical_bigger(state),
            Rule::logical_smaller_eq => rules::logical_smaller_eq(state),
            Rule::logical_bigger_eq => rules::logical_bigger_eq(state),
            Rule::logical_not_eq => rules::logical_not_eq(state),
            Rule::true_bool => rules::true_bool(state),
            Rule::false_bool => rules::false_bool(state),
            Rule::bool_lit => rules::bool_lit(state),
            Rule::factor => rules::factor(state),
            Rule::while_expr => rules::while_expr(state),
            Rule::for_expr => rules::for_expr(state),
            Rule::call_expr => rules::call_expr(state),
            Rule::range_expr => rules::range_expr(state),
            Rule::term => rules::term(state),
            Rule::expr => rules::expr(state),
            Rule::int_type => rules::int_type(state),
            Rule::bool_type => rules::bool_type(state),
            Rule::string_type => rules::string_type(state),
            Rule::object_type => rules::object_type(state),
            Rule::type_def => rules::type_def(state),
            Rule::field_decorator => rules::field_decorator(state),
            Rule::const_stmt => rules::const_stmt(state),
            Rule::let_stmt => rules::let_stmt(state),
            Rule::assignment_stmt => rules::assignment_stmt(state),
            Rule::use_stmt => rules::use_stmt(state),
            Rule::struct_stmt => rules::struct_stmt(state),
            Rule::enum_field_object_field => rules::enum_field_object_field(state),
            Rule::enum_field_object => rules::enum_field_object(state),
            Rule::enum_field_tuple => rules::enum_field_tuple(state),
            Rule::enum_field => rules::enum_field(state),
            Rule::enum_stmt => rules::enum_stmt(state),
            Rule::type_field => rules::type_field(state),
            Rule::type_object => rules::type_object(state),
            Rule::type_simple => rules::type_simple(state),
            Rule::type_stmt => rules::type_stmt(state),
            Rule::return_stmt => rules::return_stmt(state),
            Rule::break_stmt => rules::break_stmt(state),
            Rule::continue_stmt => rules::continue_stmt(state),
            Rule::function_arg => rules::function_arg(state),
            Rule::function_args => rules::function_args(state),
            Rule::function_stmt => rules::function_stmt(state),
            Rule::object_stmt_field => rules::object_stmt_field(state),
            Rule::object_stmt => rules::object_stmt(state),
            Rule::if_branch => rules::if_branch(state),
            Rule::else_if_branch => rules::else_if_branch(state),
            Rule::else_branch => rules::else_branch(state),
            Rule::if_stmt => rules::if_stmt(state),
            Rule::match_condition => rules::match_condition(state),
            Rule::match_case => rules::match_case(state),
            Rule::match_stmt => rules::match_stmt(state),
            Rule::block_stmt => rules::block_stmt(state),
            Rule::stmt => rules::stmt(state),
            Rule::stmts => rules::stmts(state),
            Rule::file => rules::file(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
