use crate::compile::lex::Token;

enum ASTtype {
    Super,
}

struct AST {
    token: Token,
    tp: ASTtype,
}
