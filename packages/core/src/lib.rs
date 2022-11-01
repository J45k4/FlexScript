use std::{vec, any};

use parser_gen::Rule;
use pest::{Parser, iterators::Pairs};

mod ast;
mod parser_gen;
mod parser_tests;
mod ast_parsing_tests;
mod parser;
mod compiler;
mod vm;
mod vm_types;

pub use ast::*;

pub struct FlexscriptParser;

pub use parser::parse_raw_ast;
pub use parser::parse_file;