use crate::compile::ast::{ASTtype, AST};
use std::cell::RefCell;
use std::io::BufReader;
use std::io::SeekFrom;
use std::thread_local;
use std::{fs::File, io::Seek};

use super::super::lex::{Lex, TokenType};

/*
function naming rule

fn (objective to parse)_(current progress)
*/

enum ParseStatus {
    Success,
    Error,
    EOF,
    EOFError,
    UnexpectedToken,
}

/*
fn main_parse(f: &mut BufReader<File>) -> (AST, ParseStatus) {
    let tokbuf = f.lex();
    let mut retast = AST::new();
    let mut status = ParseStatus::Success;
    let save = f.seek(SeekFrom::Current(0));

    if let Some(token) = tokbuf {
        match token.tp {
            TokenType::String => {
                status = parse_string(f, &mut retast);
            }
            _ => {}
        }
        return (retast, status);
    } else {
        return (retast, ParseStatus::EOF);
    }
    return (retast, ParseStatus::Error);
}

fn parse_string(f: &mut BufReader<File>, ast: &mut AST) -> ParseStatus {
    let tokbuf = f.lex();

    if let Some(token) = tokbuf {
        match token.tp {
            TokenType::LeftParen => {
                ast.tp = ASTtype::CallFunction;
                ast.token = token;
                return callfunction_arg_start(f, ast);
            }
            _ => {
                return ParseStatus::UnexpectedToken;
            }
        }
    } else {
        return ParseStatus::EOFError;
    }
}

fn callfunction_arg_start(f: &mut BufReader<File>, ast: &mut AST) -> ParseStatus {
    let tokbuf = f.lex();

    if let Some(token) = tokbuf {
        match token.tp {
            TokenType::RightParen => {
                return callfunction_end(f, ast);
            }
            TokenType::String => {
                ast.push_next(ASTtype::CallFunction_arg, token);
                return callfunction_arg_comma(f, ast);
            }
            _ => {
                return ParseStatus::UnexpectedToken;
            }
        }
    } else {
        return ParseStatus::EOFError;
    }
}

fn callfunction_arg_comma(f: &mut BufReader<File>, ast: &mut AST) -> ParseStatus {
    let tokbuf = f.lex();

    if let Some(token) = tokbuf {
        match token.tp {
            TokenType::Comma => {
                return callfunction_arg_comma(f, ast);
            }
            TokenType::RightParen => {
                return callfunction_end(f, ast);
            }
            _ => {
                return ParseStatus::UnexpectedToken;
            }
        }
    } else {
        return ParseStatus::EOFError;
    }
}
*/
