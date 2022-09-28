use std::fs::File;
use std::io::BufReader;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use super::lex::Lex;

use super::lex::Token;

#[derive(Clone)]
pub enum ASTtype {
    Super,
    CallFunction,
    CallFunction_arg,
    Bunch,
    Fucntion,
    FnIdent,
    FnRetType,
    CodeBlock,
    Match,
    MatchCase,
    If,
    For,
    While,
    Return,
    Trait,
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

    //pub fn push_next(self, tp: ASTtype, t: Token) -> &'static mut AST {}
}

/*
CallFunction -> name, args..., next


Fucntion      -> 4 entry (name, args, return type, code, next)
CodeBlock -> ? entry (codes...)
Match   -> ? entry (value, cases...)
MatchCase    -> 3 entry (value, code)
If      -> 2 entry (code, next)
For     -> 2 entry (variable, range, code)
While   -> 2 entry (condition, code)
Return  -> 1 entry (value)

etc     -> 1 entry (next)
*/
