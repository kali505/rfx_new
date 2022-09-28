use std::slice::Iter;
use std::{fs::File, io::BufReader};

use super::{
    ast::ASTtype,
    ast::AST,
    lex::{Lex, Token, TokenType},
};

mod parse;
/*
struct SyntaxInfo<'a> {
    expect: Vec<Option<TokenType>>,
    result: Vec<Option<AST>>,
    done: Vec<Option<bool>>,
    tok_iter: Iter<'a, Token>,
    err: bool,
    end: bool,
}

impl<'a> SyntaxInfo<'a> {
    fn new(iter: Iter<'a, Token>) -> Self {
        return Self {
            expect: Vec::new(),
            result: Vec::new(),
            done: Vec::new(),
            tok_iter: iter,
            err: false,
            end: false,
        };
    }

    fn gettok(&self) -> Option<&Token> {
        return self.tok_iter.next();
    }
}

//super means cannot determine
static start_expect_list: &[(TokenType, ASTtype, &[usize])] =
    &[(TokenType::Fn, ASTtype::Fucntion, &[0])];

type SyntaxASTAdder = fn(&mut AST, Token);

static next_expect_list: &[(TokenType, Option<SyntaxASTAdder>, &[usize])] = &[
    (TokenType::String, syntax_ast_fn_name_adder, &[1]), //0
    (TokenType::LeftParen, None, &[2, 3]),               //1
    (TokenType::RightParen, None, &[5, 7]),              //2
    (TokenType::String, syntax_ast_fn_arg_adder, &[4]),  //3
    (TokenType::Comma, None, &[2, 3]),                   //4
    (TokenType::Arrow, None, &[6]),                      //5
    (TokenType::String, syntax_ast_fn_return_adder, &[7]), //6
    (TokenType::LeftBrack, None, &[]),                   //7
];

trait SyntaxCheck<'a> {
    fn start(i: Iter<'a, Token>) -> SyntaxInfo<'a>;
    fn get_next_expect(
        &'a self,
        index: usize,
        out_toktp: &'a mut Vec<Option<TokenType>>,
        out_ast: &'a mut Vec<Option<AST>>,
        out_done: &mut Vec<Option<bool>>,
    );
    fn check_next(&'a self) -> SyntaxInfo<'a>;
    fn ignore_next(&'a self) -> SyntaxInfo<'a>;
    fn peek_next(&'a self) -> SyntaxInfo<'a>;
    //return result ast. if "self.result.len != 1" return self.result.len
    fn get_result(&self) -> Result<AST, u64>;
}

impl<'a> SyntaxCheck<'a> for SyntaxInfo<'a> {
    fn start(i: Iter<'a, Token>) -> SyntaxInfo<'a> {
        return SyntaxInfo::new(i);
    }
    fn get_next_expect(
        &'a self,
        index: usize,
        out_toktp: &mut Vec<Option<TokenType>>,
        out_ast: &mut Vec<Option<AST>>,
        out_done: &mut Vec<Option<bool>>,
    ) {
        let now_ast_root = self.result.iter().nth(index).unwrap();

        if now_ast_root.is_none() {
            return;
        }

        let now_ast = now_ast_root.unwrap();
        match now_ast.tp {
            _ => {
                return;
            }
        }
    }
    fn check_next(&'a self) -> SyntaxInfo<'a> {
        let tok = self.gettok();
        let del: Vec<usize> = Vec::new();
        if tok.is_none() {
            self.end = true;
            return *self;
        }

        let tok = tok.unwrap();
        let new_expect: Vec<Option<TokenType>> = Vec::new();
        let new_result: Vec<Option<AST>> = Vec::new();
        let new_done: Vec<Option<bool>> = Vec::new();
        for ((i, expect), done) in self.expect.iter().enumerate().zip(self.done.iter()) {
            if expect.is_none() {
                continue;
            }
            if expect.unwrap() != tok.tp || done.unwrap() {
                *expect = None;
                continue;
            }

            self.get_next_expect(i, &mut new_expect, &mut new_result, &mut new_done);
        }

        self.expect = new_expect;
        self.result = new_result;
        self.done = new_done;

        return *self;
    }
    fn ignore_next(&'a self) -> SyntaxInfo<'a> {}
    fn peek_next(&'a self) -> SyntaxInfo<'a> {}
    //return result ast. if "self.result.len != 1" return self.result.len
    fn get_result(&self) -> Result<AST, u64> {}
}

/*
fn try_parse_func(v: Iter<Token>) -> Option<AST> {
    let ret: AST = AST::new();

    ret.tp = ASTtype::Fucntion;

    let tok = v.next();
    let expect = TokenType::Fn;
    if tok.is_none() || tok.unwrap().tp == expect {
        return None;
    }

    let tok = v.next();
    let expect = TokenType::String;
    if tok.is_none() || tok.unwrap().tp == expect {
        return None;
    } else {
        //push name
        let name_ast = AST::new();
        name_ast.tp = ASTtype::FnIdent;
        name_ast.token = tok.unwrap().clone();
        ret.next.push(name_ast);
    }

    let tok = v.next();
    let expect = TokenType::LeftParen;
    if tok.is_none() || tok.unwrap().tp == expect {
        return None;
    }

    //push arg
    let args = try_parse_func_arg(v);
    ret.next.push(args);

    //fn abc(...) -> type {
    // or
    //fn abc(...) {
    let tok = v.next();
    if tok.is_none() {
        return None;
    }
    let tok = tok.unwrap();
    let rettype_ast = AST::new();
    rettype_ast.tp = ASTtype::FnRetType;
    ret.next.push(rettype_ast);
    if tok.tp == TokenType::Arrow {
        //push return type
        let rettype_tok = v.next();
        if rettype_tok.is_none() {
            return None;
        }
        let rettype_tok = rettype_tok.unwrap();
        if rettype_tok.tp != TokenType::String {
            return None;
        }
        rettype_ast.token = *rettype_tok;
    } else if tok.tp == TokenType::LeftBrace {
        //push dummy return type
        rettype_ast.token = Token::new();
        rettype_ast.token.tp = TokenType::Null;
    }

    return Some(ret);
}

//none means EOF
pub fn get_AST_block(r: BufReader<File>) -> Option<AST> {
    let tok_vec: Vec<Token> = Vec::new();

    loop {
        let tok_wrap = r.lex();
        if tok_wrap.is_none() {
            break;
        }

        let tok = tok_wrap.unwrap();
        if tok.tp == TokenType::LeftBrace || tok.tp == TokenType::Semicolon {
            tok_vec.push(tok);
            break;
        }
        tok_vec.push(tok);
    }

    try_parse_func(tok_vec.iter());
}
 */
*/
