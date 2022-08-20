use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use super::lex::Lex;

use super::lex::Token;

#[derive(Clone)]
pub enum ASTtype {
    Super,
    Fucntion,
    CodeBlock,
    Match,
    MatchCase,
    Return,
    VariableAttr, // const, static...
    VariableType,
    VariableName,
}

#[derive(Clone)]
pub struct AST {
    pub token: Token,
    pub tp: ASTtype,
    pub next: Vec<AST>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            token: Token::new(),
            tp: ASTtype::Super,
            next: Vec::new(),
        }
    }
}

/*
Fucntion      -> 4 entry (return type, name, args, code, next)
CodeBlock -> ? entry (codes...)
Match   -> ? entry (value, cases...)
MatchCase    -> 3 entry (value, code)
if      -> 2 entry (code, next)
for     -> 2 entry (variable, range, code)
while   -> 2 entry (condition, code)
return  -> 1 entry (value)

etc     -> 1 entry (next)
*/

// main -> build_rfx_ast -> parse AST
fn build_rfx_ast(f: &mut BufReader<File>) -> Result<AST, &str> {
    let mut ret = AST::new();
    let t = f.lex();
    let unwrap_t;

    if t.is_none() {
        return Err("Unexpected EOF.");
    }

    unwrap_t = t.unwrap();
    match unwrap_t.tp {
        _ => {}
    }

    return Ok(ret);
}
